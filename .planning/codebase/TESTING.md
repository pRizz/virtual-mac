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
    index.ts           # Barrel export
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

**Structure:**
```typescript
import { Page, Locator } from '@playwright/test';

export class DockPage {
  readonly page: Page;
  readonly dockContainer: Locator;
  readonly dock: Locator;
  readonly dockItems: Locator;

  constructor(page: Page) {
    this.page = page;
    this.dockContainer = page.locator('.dock-container');
    this.dock = page.locator('.dock-container .dock');
    this.dockItems = page.locator('.dock-container .dock-item');
  }

  getDockItem(appName: string): Locator {
    return this.dock.locator(`.dock-item[data-tooltip="${appName}"]`);
  }

  async hoverDockItem(appName: string) {
    const item = this.getDockItem(appName);
    await item.hover();
    return item;
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

**Keyboard Shortcuts:**
```typescript
test('Cmd+Q should close all windows', async ({ page }) => {
  await page.keyboard.press('Meta+q');
  await expect(finder).not.toBeVisible();
});
```

**Class Assertions:**
```typescript
await expect(finder).toHaveClass(/maximized/);
await expect(finder).not.toHaveClass(/maximized/);
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

## Important Notes

**E2E Test Moratorium:**
Per `CLAUDE.md`, new E2E tests should NOT be added until the E2E test cleanup (vi-t5a) is complete. Focus on unit tests if coverage is needed.

**Test Execution:**
- Tests require Trunk installed (`trunk serve`)
- WASM compilation can be slow; webServer timeout set to 120s
- CI runs with single worker and 2 retries

---

*Testing analysis: 2026-01-17*
