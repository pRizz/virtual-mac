import { Page, Locator } from '@playwright/test';

export class FinderPage {
  readonly page: Page;
  readonly finder: Locator;
  readonly toolbar: Locator;
  readonly sidebar: Locator;
  readonly contentArea: Locator;
  readonly fileGrid: Locator;
  readonly statusBar: Locator;
  readonly searchInput: Locator;
  readonly viewButtons: Locator;
  readonly navBackButton: Locator;
  readonly navForwardButton: Locator;

  constructor(page: Page, windowLocator: Locator) {
    this.page = page;
    this.finder = windowLocator.locator('.finder');
    this.toolbar = this.finder.locator('.finder-toolbar');
    this.sidebar = this.finder.locator('.finder-sidebar');
    this.contentArea = this.finder.locator('.finder-content');
    this.fileGrid = this.finder.locator('.finder-grid');
    this.statusBar = this.finder.locator('.finder-statusbar');
    this.searchInput = this.finder.locator('.finder-search-input');
    this.viewButtons = this.finder.locator('.finder-view-btn');
    this.navBackButton = this.finder.locator('.finder-nav-btn').first();
    this.navForwardButton = this.finder.locator('.finder-nav-btn').nth(1);
  }

  getSidebarItem(name: string): Locator {
    return this.sidebar.locator('.sidebar-item', { hasText: name });
  }

  getFileItem(name: string): Locator {
    return this.fileGrid.locator('.finder-item', { hasText: name });
  }

  async navigateToSidebarItem(name: string) {
    await this.getSidebarItem(name).click();
  }

  async selectFile(name: string) {
    await this.getFileItem(name).click();
  }

  async isFileSelected(name: string): Promise<boolean> {
    const item = this.getFileItem(name);
    return item.evaluate((el) => el.classList.contains('selected'));
  }

  async isSidebarItemSelected(name: string): Promise<boolean> {
    const item = this.getSidebarItem(name);
    return item.evaluate((el) => el.classList.contains('selected'));
  }

  async getToolbarTitle(): Promise<string | null> {
    return this.toolbar.locator('.finder-toolbar-title').textContent();
  }

  async getStatusBarText(): Promise<string | null> {
    return this.statusBar.textContent();
  }

  async getFileCount(): Promise<number> {
    return this.fileGrid.locator('.finder-item').count();
  }

  async clickViewButton(index: number) {
    await this.viewButtons.nth(index).click();
  }

  async isViewButtonActive(index: number): Promise<boolean> {
    const button = this.viewButtons.nth(index);
    return button.evaluate((el) => el.classList.contains('active'));
  }
}
