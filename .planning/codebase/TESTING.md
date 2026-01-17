# Testing Patterns

**Analysis Date:** 2026-01-17

## Test Framework

**Runner:**
- Playwright 1.48.0
- Config: `playwright.config.ts`
- TypeScript 5.6.0 for test code

**Assertion Library:**
- Playwright's built-in `expect` from `@playwright/test`

**Run Commands:**
```bash
npm test                 # Run all tests
npm run test:headed      # Run with browser visible
npm run test:debug       # Run in debug mode
npm run test:ui          # Open Playwright UI
npm run test:report      # View HTML report
```

## Test File Organization

**Location:**
- Specs: `e2e/specs/*.spec.ts`
- Page Objects: `e2e/page-objects/*.page.ts`
- Index export: `e2e/page-objects/index.ts`

**Naming:**
- Spec files: `{feature}.spec.ts` (e.g., `calculator.spec.ts`, `dock.spec.ts`)
- Page objects: `{feature}.page.ts` (e.g., `calculator.page.ts`, `dock.page.ts`)

**Structure:**
```
e2e/
  page-objects/
    index.ts               # Barrel export
    calculator.page.ts
    desktop.page.ts
    dock.page.ts
    finder.page.ts
    menu-bar.page.ts
    window-manager.page.ts
  specs/
    calculator.spec.ts
    desktop.spec.ts
    dock.spec.ts
    finder.spec.ts
    menu-bar.spec.ts
    window-manager.spec.ts
```

## Test Structure

**Suite Organization:**
```typescript
import { test, expect } from '@playwright/test';
import { DockPage, DesktopPage } from '../page-objects';

test.describe('Dock', () => {
  let dock: DockPage;

  test.beforeEach(async ({ page }) => {
    const desktop = new DesktopPage(page);
    await desktop.goto();
    dock = new DockPage(page);
  });

  test.describe('Dock Display', () => {
    test('should display dock at bottom of screen', async () => {
      await expect(dock.dockContainer).toBeVisible();
      // assertions...
    });
  });
});
```

**Patterns:**
- Nested `test.describe()` for feature grouping
- `test.beforeEach()` for common setup (navigation, page object init)
- Page objects instantiated per test suite
- Descriptive test names: `'should display all dock icons'`

## Page Object Pattern

**Base Structure (example from `e2e/page-objects/dock.page.ts`):**
```typescript
import { Page, Locator } from '@playwright/test';

export class DockPage {
  readonly page: Page;
  readonly dockContainer: Locator;
  readonly dock: Locator;
  readonly dockItems: Locator;
  readonly dockSeparator: Locator;

  constructor(page: Page) {
    this.page = page;
    this.dockContainer = page.locator('.dock-container');
    this.dock = page.locator('.dock-container .dock');
    this.dockItems = page.locator('.dock-container .dock-item');
    this.dockSeparator = page.locator('.dock-separator');
  }

  getDockItem(appName: string): Locator {
    return this.dock.locator(`.dock-item[data-tooltip="${appName}"]`);
  }

  async hoverDockItem(appName: string) {
    const item = this.getDockItem(appName);
    await item.hover();
    return item;
  }

  async clickDockItem(appName: string) {
    await this.getDockItem(appName).click();
  }
}
```

**Window-Scoped Page Objects (example from `e2e/page-objects/finder.page.ts`):**
```typescript
export class FinderPage {
  readonly page: Page;
  readonly finder: Locator;
  readonly toolbar: Locator;
  readonly sidebar: Locator;

  constructor(page: Page, windowLocator: Locator) {
    this.page = page;
    this.finder = windowLocator.locator('.finder');
    this.toolbar = this.finder.locator('.finder-toolbar');
    this.sidebar = this.finder.locator('.finder-sidebar');
  }
}
```

**Conventions:**
- `readonly` for locators set in constructor
- Locator factory methods: `getWindow(title)`, `getDockItem(name)`
- Action methods: `async hoverDockItem()`, `async clickDockItem()`
- Query methods: `async getDisplayValue()`, `async getItemScale()`

## Locator Patterns

**CSS Selectors:**
```typescript
// By class
page.locator('.dock-container')

// By data attribute
this.dock.locator(`.dock-item[data-tooltip="${appName}"]`)

// Combined class selectors
this.buttons.locator('.calc-btn.operator', { hasText: /^=$/ })
```

**Filtering:**
```typescript
// Filter by child element
this.page.locator('.window').filter({
  has: this.page.locator('.window-title', { hasText: title })
});
```

**Text Matching:**
```typescript
// Exact text with regex
{ hasText: new RegExp(`^${escapedDigit}$`) }

// Contains text
{ hasText: 'Finder' }

// Escape special characters for regex
const escapedDigit = digit.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
```

## Mocking

**Framework:** Not used - tests run against real WASM app

**Approach:**
- No mocking framework configured
- Tests use actual application with Trunk dev server
- WebServer auto-started via `trunk serve --port 8080`

**What NOT to Mock:**
- Application state
- DOM interactions
- WASM module

**External Dependencies:**
- LocalStorage interactions happen naturally
- No external API calls to mock

## Fixtures and Factories

**Test Data:**
- Hard-coded test data in specs
- No dedicated fixture files

**Pattern:**
```typescript
const icons = ['Finder', 'Safari', 'Messages', 'Mail', 'Photos',
               'Music', 'Notes', 'Calendar', 'System Settings',
               'Terminal', 'Downloads', 'Trash'];
for (const icon of icons) {
  await expect(dock.getDockItem(icon)).toBeVisible();
}
```

**Location:**
- Test data inline in spec files
- Page objects contain element selectors, not test data

## Coverage

**Requirements:** None enforced

**Note:** E2E tests only; no unit test coverage for Rust code.

## Test Types

**E2E Tests:**
- All tests are end-to-end Playwright tests
- Tests run against compiled WASM app in browser
- Multi-browser: Chromium, Firefox, WebKit

**Unit Tests:**
- No Rust unit tests detected in `src/`
- No `#[cfg(test)]` blocks in source files

**Integration Tests:**
- E2E tests serve as integration tests
- Test user flows through multiple components

## Common Patterns

**Async Testing:**
```typescript
test('should magnify icon on hover', async () => {
  await dock.hoverDockItem('Finder');
  await dock.page.waitForTimeout(200);  // Wait for animation

  const hoverScale = await dock.getItemScale('Finder');
  expect(hoverScale).toBeGreaterThanOrEqual(1);
});
```

**DOM Evaluation:**
```typescript
async getItemScale(appName: string): Promise<number> {
  const item = this.getDockItem(appName);
  const transform = await item.evaluate((el) => {
    return window.getComputedStyle(el).transform;
  });
  if (transform === 'none') return 1;
  const match = transform.match(/matrix\(([^,]+)/);
  return match ? parseFloat(match[1]) : 1;
}
```

**Class State Checking:**
```typescript
async isWindowActive(windowLocator: Locator) {
  return windowLocator.evaluate((el) => el.classList.contains('active'));
}

async isRunningIndicatorVisible(appName: string): Promise<boolean> {
  const item = this.getDockItem(appName);
  const indicator = item.locator('.dock-indicator');
  return indicator.evaluate((el) => el.classList.contains('active'));
}
```

**Mouse Interactions:**
```typescript
async dragWindow(windowLocator: Locator, deltaX: number, deltaY: number) {
  const titleBar = this.getTitleBar(windowLocator);
  const box = await titleBar.boundingBox();
  if (!box) throw new Error('Window not found');

  const startX = box.x + box.width / 2;
  const startY = box.y + box.height / 2;

  await this.page.mouse.move(startX, startY);
  await this.page.mouse.down();
  await this.page.mouse.move(startX + deltaX, startY + deltaY);
  await this.page.mouse.up();
}
```

**Resize Testing:**
```typescript
async resizeWindow(
  windowLocator: Locator,
  handle: string,
  deltaX: number,
  deltaY: number
) {
  const resizeHandle = this.getResizeHandle(windowLocator, handle);
  const box = await resizeHandle.boundingBox();
  if (!box) throw new Error('Resize handle not found');

  await this.page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await this.page.mouse.down();
  await this.page.mouse.move(box.x + deltaX, box.y + deltaY);
  await this.page.mouse.up();
}
```

**Selection Rectangle Drawing:**
```typescript
async drawSelectionRectangle(
  startX: number,
  startY: number,
  endX: number,
  endY: number
) {
  await this.desktop.hover({ position: { x: startX, y: startY } });
  await this.page.mouse.down();
  await this.page.mouse.move(endX, endY);
  return this.selectionRect;
}
```

**Class Assertions:**
```typescript
await expect(finder).toHaveClass(/maximized/);
await expect(finder).not.toHaveClass(/maximized/);
await expect(menuBar.appleMenu).toHaveClass(/active/);
```

**Bounding Box Assertions:**
```typescript
test('should display dock at bottom of screen', async () => {
  const bounds = await dock.dock.boundingBox();
  const viewportSize = await dock.page.viewportSize();

  expect(bounds!.y + bounds!.height).toBeGreaterThan(viewportSize!.height - 100);
});

test('should display menu bar at top of screen', async () => {
  const bounds = await menuBar.menuBar.boundingBox();
  expect(bounds!.y).toBe(0);
});
```

**Text Content Assertions:**
```typescript
test('should show live clock with time format', async () => {
  const clockText = await menuBar.getClockText();
  expect(clockText).toBeTruthy();
  expect(clockText!.length).toBeGreaterThan(0);
});

test('should display item count', async () => {
  const statusText = await finder.getStatusBarText();
  expect(statusText).toMatch(/\d+ items?/);
});
```

**Calculator Expression Helper:**
```typescript
async calculate(expression: string) {
  const match = expression.match(/^(\d+(?:\.\d+)?)([\+\-\*\/×÷])(\d+(?:\.\d+)?)$/);
  if (!match) throw new Error(`Invalid expression: ${expression}`);

  const [, num1, op, num2] = match;

  await this.pressClear();
  for (const digit of num1) {
    if (digit === '.') await this.pressDecimal();
    else await this.pressDigit(digit);
  }

  const opMap: Record<string, string> = { '+': '+', '-': '−', '*': '×', '/': '÷' };
  await this.pressOperator(opMap[op] || op);

  for (const digit of num2) {
    if (digit === '.') await this.pressDecimal();
    else await this.pressDigit(digit);
  }

  await this.pressEquals();
}
```

**Bringing Window to Front (JavaScript evaluation):**
```typescript
await calcWindow.evaluate((el) => {
  // Dispatch mousedown event to trigger bring_to_front
  el.dispatchEvent(new MouseEvent('mousedown', { bubbles: true }));
});
```

## Playwright Configuration

**Key Settings from `playwright.config.ts`:**
```typescript
export default defineConfig({
  testDir: './e2e/specs',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [['html'], ['list']],
  use: {
    baseURL: 'http://localhost:8080',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'on-first-retry',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
    { name: 'webkit', use: { ...devices['Desktop Safari'] } },
  ],
  webServer: {
    command: 'trunk serve --port 8080',
    url: 'http://localhost:8080',
    reuseExistingServer: !process.env.CI,
    timeout: 120000,
  },
});
```

## Adding New Tests

**When to Add Tests:**
Per `CLAUDE.md`, new E2E tests should NOT be added until the E2E test cleanup (vi-t5a) is complete.

**If Adding Tests (Post-Moratorium):**

1. Create page object in `e2e/page-objects/{feature}.page.ts`:
```typescript
import { Page, Locator } from '@playwright/test';

export class NewFeaturePage {
  readonly page: Page;
  readonly mainElement: Locator;

  constructor(page: Page) {
    this.page = page;
    this.mainElement = page.locator('.new-feature');
  }

  async someAction() {
    // ...
  }
}
```

2. Export from `e2e/page-objects/index.ts`:
```typescript
export { NewFeaturePage } from './new-feature.page';
```

3. Create spec in `e2e/specs/{feature}.spec.ts`:
```typescript
import { test, expect } from '@playwright/test';
import { NewFeaturePage, DesktopPage } from '../page-objects';

test.describe('New Feature', () => {
  let feature: NewFeaturePage;

  test.beforeEach(async ({ page }) => {
    const desktop = new DesktopPage(page);
    await desktop.goto();
    feature = new NewFeaturePage(page);
  });

  test('should do something', async () => {
    // ...
  });
});
```

## Test Execution

**Requirements:**
- Node.js with npm
- Trunk installed (`cargo install trunk`)
- WASM compilation can be slow; webServer timeout set to 120s

**CI Configuration:**
- CI runs with single worker and 2 retries
- `forbidOnly: true` prevents `.only` tests from passing CI

**Artifacts:**
- HTML report: `playwright-report/`
- Test results: `test-results/`

---

*Testing analysis: 2026-01-17*
