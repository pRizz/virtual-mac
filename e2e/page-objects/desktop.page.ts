import { Page, Locator } from '@playwright/test';

export class DesktopPage {
  readonly page: Page;
  readonly desktop: Locator;
  readonly selectionRect: Locator;

  constructor(page: Page) {
    this.page = page;
    this.desktop = page.locator('.desktop').first();
    this.selectionRect = page.locator('.selection-rect');
  }

  async goto() {
    await this.page.goto('/');
    await this.page.waitForSelector('.desktop');
  }

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

  async releaseSelection() {
    await this.page.mouse.up();
  }

  async getSelectionRectBounds() {
    return this.selectionRect.boundingBox();
  }
}
