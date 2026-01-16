import { Page, Locator } from '@playwright/test';

export class WindowManagerPage {
  readonly page: Page;

  constructor(page: Page) {
    this.page = page;
  }

  getWindow(title: string): Locator {
    return this.page.locator('.window').filter({
      has: this.page.locator('.window-title', { hasText: title })
    });
  }

  getTitleBar(windowLocator: Locator): Locator {
    return windowLocator.locator('.window-titlebar');
  }

  getCloseButton(windowLocator: Locator): Locator {
    return windowLocator.locator('.traffic-light.close');
  }

  getMinimizeButton(windowLocator: Locator): Locator {
    return windowLocator.locator('.traffic-light.minimize');
  }

  getMaximizeButton(windowLocator: Locator): Locator {
    return windowLocator.locator('.traffic-light.maximize');
  }

  getResizeHandle(windowLocator: Locator, direction: string): Locator {
    return windowLocator.locator(`.resize-handle.${direction}`);
  }

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

  async resizeWindow(
    windowLocator: Locator,
    handle: string,
    deltaX: number,
    deltaY: number
  ) {
    const resizeHandle = this.getResizeHandle(windowLocator, handle);
    const box = await resizeHandle.boundingBox();
    if (!box) throw new Error('Resize handle not found');

    const startX = box.x + box.width / 2;
    const startY = box.y + box.height / 2;

    await this.page.mouse.move(startX, startY);
    await this.page.mouse.down();
    await this.page.mouse.move(startX + deltaX, startY + deltaY);
    await this.page.mouse.up();
  }

  async getWindowPosition(windowLocator: Locator) {
    return windowLocator.boundingBox();
  }

  async isWindowActive(windowLocator: Locator) {
    return windowLocator.evaluate((el) => el.classList.contains('active'));
  }

  async isWindowMaximized(windowLocator: Locator) {
    return windowLocator.evaluate((el) => el.classList.contains('maximized'));
  }

  async isWindowMinimized(windowLocator: Locator) {
    return windowLocator.evaluate((el) => el.classList.contains('minimized'));
  }

  async getZIndex(windowLocator: Locator) {
    return windowLocator.evaluate((el) => {
      return parseInt(window.getComputedStyle(el).zIndex, 10);
    });
  }
}
