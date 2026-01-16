import { test, expect } from '@playwright/test';
import { MenuBarPage, DesktopPage } from '../page-objects';

test.describe('Menu Bar', () => {
  let menuBar: MenuBarPage;

  test.beforeEach(async ({ page }) => {
    const desktop = new DesktopPage(page);
    await desktop.goto();
    menuBar = new MenuBarPage(page);
  });

  test.describe('Menu Bar Layout', () => {
    test('should display menu bar at top of screen', async () => {
      await expect(menuBar.menuBar).toBeVisible();
      const bounds = await menuBar.menuBar.boundingBox();
      expect(bounds!.y).toBe(0);
    });

    test('should have left and right sections', async () => {
      await expect(menuBar.menuBarLeft).toBeVisible();
      await expect(menuBar.menuBarRight).toBeVisible();
    });
  });

  test.describe('Dropdown Menus', () => {
    test('should display all menus', async () => {
      await expect(menuBar.appleMenu).toBeVisible();
      await expect(menuBar.appMenu).toBeVisible();
      await expect(menuBar.fileMenu).toBeVisible();
      await expect(menuBar.editMenu).toBeVisible();
      await expect(menuBar.viewMenu).toBeVisible();
      await expect(menuBar.windowMenu).toBeVisible();
      await expect(menuBar.helpMenu).toBeVisible();
    });

    test('should open Apple menu dropdown on click', async () => {
      await menuBar.openMenu(menuBar.appleMenu);

      await expect(menuBar.appleMenu).toHaveClass(/active/);
      const dropdown = menuBar.appleMenu.locator('.menu-dropdown');
      await expect(dropdown).toBeVisible();
    });

    test('should show About This Mac in Apple menu', async () => {
      await menuBar.openMenu(menuBar.appleMenu);

      const items = await menuBar.getDropdownItems(menuBar.appleMenu);
      await expect(items.filter({ hasText: 'About This Mac' })).toBeVisible();
    });

    test('should show System Settings in Apple menu', async () => {
      await menuBar.openMenu(menuBar.appleMenu);

      const items = await menuBar.getDropdownItems(menuBar.appleMenu);
      await expect(items.filter({ hasText: 'System Settings' })).toBeVisible();
    });

    test('should show File menu items', async () => {
      await menuBar.openMenu(menuBar.fileMenu);

      const dropdown = menuBar.fileMenu.locator('.menu-dropdown');
      await expect(dropdown).toBeVisible();
    });

    test('should close menu when clicking elsewhere', async ({ page }) => {
      await menuBar.openMenu(menuBar.fileMenu);
      await expect(menuBar.fileMenu).toHaveClass(/active/);

      await page.mouse.click(500, 300);
      await expect(menuBar.fileMenu).not.toHaveClass(/active/);
    });

    test('should switch menus on hover when one is open', async () => {
      await menuBar.openMenu(menuBar.fileMenu);
      await menuBar.editMenu.hover();

      await expect(menuBar.editMenu).toHaveClass(/active/);
      await expect(menuBar.fileMenu).not.toHaveClass(/active/);
    });
  });

  test.describe('Status Icons', () => {
    test('should display WiFi icon', async () => {
      await expect(menuBar.wifiIcon).toBeVisible();
    });

    test('should display battery icon', async () => {
      await expect(menuBar.batteryIcon).toBeVisible();
    });

    test('should display clock', async () => {
      await expect(menuBar.clock).toBeVisible();
    });

    test('should show live clock with time format', async () => {
      const clockText = await menuBar.getClockText();
      expect(clockText).toBeTruthy();
      expect(clockText!.length).toBeGreaterThan(0);
    });

    test('should display battery percentage', async () => {
      const percent = await menuBar.getBatteryPercent();
      expect(percent).toMatch(/\d+%/);
    });
  });
});
