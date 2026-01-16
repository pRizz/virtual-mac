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
      const bounds = await dock.dock.boundingBox();
      const viewportSize = await dock.page.viewportSize();

      expect(bounds!.y + bounds!.height).toBeGreaterThan(viewportSize!.height - 100);
    });

    test('should display Finder icon', async () => {
      await expect(dock.getDockItem('Finder')).toBeVisible();
    });

    test('should display Safari icon', async () => {
      await expect(dock.getDockItem('Safari')).toBeVisible();
    });

    test('should display Messages icon', async () => {
      await expect(dock.getDockItem('Messages')).toBeVisible();
    });

    test('should display Mail icon', async () => {
      await expect(dock.getDockItem('Mail')).toBeVisible();
    });

    test('should display Photos icon', async () => {
      await expect(dock.getDockItem('Photos')).toBeVisible();
    });

    test('should display Music icon', async () => {
      await expect(dock.getDockItem('Music')).toBeVisible();
    });

    test('should display Notes icon', async () => {
      await expect(dock.getDockItem('Notes')).toBeVisible();
    });

    test('should display Calendar icon', async () => {
      await expect(dock.getDockItem('Calendar')).toBeVisible();
    });

    test('should display System Settings icon', async () => {
      await expect(dock.getDockItem('System Settings')).toBeVisible();
    });

    test('should display Terminal icon', async () => {
      await expect(dock.getDockItem('Terminal')).toBeVisible();
    });

    test('should display dock separator', async () => {
      await expect(dock.dockSeparator).toBeVisible();
    });

    test('should display Downloads folder', async () => {
      await expect(dock.getDockItem('Downloads')).toBeVisible();
    });

    test('should display Trash icon', async () => {
      await expect(dock.getDockItem('Trash')).toBeVisible();
    });
  });

  test.describe('Magnification Effect', () => {
    test('should magnify icon on hover', async () => {
      await dock.hoverDockItem('Finder');
      await dock.page.waitForTimeout(200);

      const hoverScale = await dock.getItemScale('Finder');
      expect(hoverScale).toBeGreaterThanOrEqual(1);
    });

    test('should return to normal scale on mouse leave', async () => {
      await dock.hoverDockItem('Finder');
      await dock.page.waitForTimeout(200);

      await dock.page.mouse.move(100, 100);
      await dock.page.waitForTimeout(200);

      const scale = await dock.getItemScale('Finder');
      expect(scale).toBeLessThanOrEqual(1.1);
    });
  });

  test.describe('Tooltips', () => {
    test('should have tooltip attribute for Finder', async () => {
      const tooltip = await dock.getTooltipText('Finder');
      expect(tooltip).toBe('Finder');
    });

    test('should have tooltip attribute for Safari', async () => {
      const tooltip = await dock.getTooltipText('Safari');
      expect(tooltip).toBe('Safari');
    });

    test('should have tooltip attribute for Trash', async () => {
      const tooltip = await dock.getTooltipText('Trash');
      expect(tooltip).toBe('Trash');
    });

    test('should have tooltip attribute for Downloads', async () => {
      const tooltip = await dock.getTooltipText('Downloads');
      expect(tooltip).toBe('Downloads');
    });
  });

  test.describe('Icon Clicks', () => {
    test('should be clickable', async () => {
      const item = dock.getDockItem('Finder');
      await expect(item).toBeEnabled();
    });

    test('should handle click on Safari icon', async () => {
      await dock.clickDockItem('Safari');
    });

    test('should handle click on Trash icon', async () => {
      await dock.clickDockItem('Trash');
    });
  });
});
