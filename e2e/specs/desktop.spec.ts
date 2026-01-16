import { test, expect } from '@playwright/test';
import { DesktopPage } from '../page-objects';

test.describe('Desktop', () => {
  let desktop: DesktopPage;

  test.beforeEach(async ({ page }) => {
    desktop = new DesktopPage(page);
    await desktop.goto();
  });

  test.describe('Selection Rectangle', () => {
    test('should show selection rectangle when dragging on desktop', async () => {
      await desktop.drawSelectionRectangle(100, 100, 300, 250);

      await expect(desktop.selectionRect).toBeVisible();
    });

    test('should have correct dimensions based on drag distance', async () => {
      await desktop.drawSelectionRectangle(100, 100, 300, 250);

      const bounds = await desktop.getSelectionRectBounds();
      expect(bounds).not.toBeNull();
      expect(bounds!.width).toBeGreaterThan(150);
      expect(bounds!.height).toBeGreaterThan(100);
    });

    test('should hide selection rectangle on mouse up', async () => {
      await desktop.drawSelectionRectangle(100, 100, 300, 250);
      await desktop.releaseSelection();

      await expect(desktop.selectionRect).not.toBeVisible();
    });

    test('should handle reverse drag direction (right to left)', async () => {
      await desktop.drawSelectionRectangle(300, 250, 100, 100);

      await expect(desktop.selectionRect).toBeVisible();
      const bounds = await desktop.getSelectionRectBounds();
      expect(bounds!.width).toBeGreaterThan(150);
      expect(bounds!.height).toBeGreaterThan(100);
    });

    test('should update rectangle dimensions during drag', async ({ page }) => {
      const desktop = new DesktopPage(page);
      await desktop.goto();

      await desktop.desktop.hover({ position: { x: 100, y: 100 } });
      await page.mouse.down();
      await page.mouse.move(200, 200);

      let bounds = await desktop.getSelectionRectBounds();
      const firstWidth = bounds?.width || 0;

      await page.mouse.move(300, 300);
      bounds = await desktop.getSelectionRectBounds();
      const secondWidth = bounds?.width || 0;

      expect(secondWidth).toBeGreaterThan(firstWidth);

      await page.mouse.up();
    });
  });
});
