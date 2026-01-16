import { Page, Locator } from '@playwright/test';

export class MenuBarPage {
  readonly page: Page;
  readonly menuBar: Locator;
  readonly menuBarLeft: Locator;
  readonly menuBarRight: Locator;
  readonly appleMenu: Locator;
  readonly appMenu: Locator;
  readonly fileMenu: Locator;
  readonly editMenu: Locator;
  readonly viewMenu: Locator;
  readonly windowMenu: Locator;
  readonly helpMenu: Locator;
  readonly clock: Locator;
  readonly wifiIcon: Locator;
  readonly batteryIcon: Locator;

  constructor(page: Page) {
    this.page = page;
    this.menuBar = page.locator('.menu-bar');
    this.menuBarLeft = page.locator('.menu-bar-left');
    this.menuBarRight = page.locator('.menu-bar-right');
    this.appleMenu = page.locator('.menu-item.apple-menu');
    this.appMenu = page.locator('.menu-item.app-name');
    this.fileMenu = page.locator('.menu-item').filter({ hasText: 'File' });
    this.editMenu = page.locator('.menu-item').filter({ hasText: 'Edit' });
    this.viewMenu = page.locator('.menu-item').filter({ hasText: 'View' });
    this.windowMenu = page.locator('.menu-item').filter({ hasText: 'Window' });
    this.helpMenu = page.locator('.menu-item').filter({ hasText: 'Help' });
    this.clock = page.locator('.status-clock');
    this.wifiIcon = page.locator('.wifi-icon');
    this.batteryIcon = page.locator('.battery-icon');
  }

  async openMenu(menuLocator: Locator) {
    await menuLocator.click();
  }

  async getDropdownItems(menuLocator: Locator) {
    return menuLocator.locator('.menu-dropdown .dropdown-item');
  }

  async clickDropdownItem(menuLocator: Locator, itemText: string) {
    await menuLocator.click();
    await menuLocator.locator('.dropdown-item', { hasText: itemText }).click();
  }

  async isMenuActive(menuLocator: Locator) {
    return menuLocator.evaluate((el) => el.classList.contains('active'));
  }

  async getClockText() {
    return this.clock.textContent();
  }

  async getBatteryPercent() {
    return this.page.locator('.battery-percent').textContent();
  }
}
