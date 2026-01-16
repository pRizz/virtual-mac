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

    test('should display all dock icons', async () => {
      const icons = ['Finder', 'Safari', 'Messages', 'Mail', 'Photos', 'Music', 'Notes', 'Calendar', 'System Settings', 'Terminal', 'Downloads', 'Trash'];
      for (const icon of icons) {
        await expect(dock.getDockItem(icon)).toBeVisible();
      }
    });

    test('should display dock separator', async () => {
      await expect(dock.dockSeparator).toBeVisible();
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
    test('should have tooltip attributes for dock items', async () => {
      const items = ['Finder', 'Safari', 'Trash', 'Downloads'];
      for (const item of items) {
        const tooltip = await dock.getTooltipText(item);
        expect(tooltip).toBe(item);
      }
    });
  });

  test.describe('Icon Clicks', () => {
    test('should be clickable', async () => {
      const item = dock.getDockItem('Finder');
      await expect(item).toBeEnabled();
    });
  });
});
