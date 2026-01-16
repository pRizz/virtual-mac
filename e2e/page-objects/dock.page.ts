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

  async getItemScale(appName: string): Promise<number> {
    const item = this.getDockItem(appName);
    const transform = await item.evaluate((el) => {
      return window.getComputedStyle(el).transform;
    });
    if (transform === 'none') return 1;
    const match = transform.match(/matrix\(([^,]+)/);
    return match ? parseFloat(match[1]) : 1;
  }

  async isRunningIndicatorVisible(appName: string): Promise<boolean> {
    const item = this.getDockItem(appName);
    const indicator = item.locator('.dock-indicator');
    return indicator.evaluate((el) => el.classList.contains('active'));
  }

  async getTooltipText(appName: string): Promise<string | null> {
    const item = this.getDockItem(appName);
    return item.getAttribute('data-tooltip');
  }

  async hoverAcrossDock() {
    const box = await this.dock.boundingBox();
    if (!box) throw new Error('Dock not found');

    const steps: { x: number; y: number }[] = [];
    for (let x = box.x; x <= box.x + box.width; x += 20) {
      steps.push({ x, y: box.y + box.height / 2 });
    }
    return steps;
  }
}
