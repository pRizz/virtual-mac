import { test, expect } from '@playwright/test';
import { WindowManagerPage, DesktopPage } from '../page-objects';

test.describe('Window Manager', () => {
  let windowManager: WindowManagerPage;

  test.beforeEach(async ({ page }) => {
    const desktop = new DesktopPage(page);
    await desktop.goto();
    windowManager = new WindowManagerPage(page);
  });

  test.describe('Window Display', () => {
    test('should show Finder window', async () => {
      const finder = windowManager.getWindow('Finder');
      await expect(finder).toBeVisible();
    });

    test('should show Calculator window', async () => {
      const calculator = windowManager.getWindow('Calculator');
      await expect(calculator).toBeVisible();
    });

    test('should show Notes window', async () => {
      const notes = windowManager.getWindow('Notes');
      await expect(notes).toBeVisible();
    });

    test('should display window title in title bar', async () => {
      const finder = windowManager.getWindow('Finder');
      const title = finder.locator('.window-title');
      await expect(title).toContainText('Finder');
    });
  });

  test.describe('Window Dragging', () => {
    test('should move window when dragging title bar', async () => {
      const finder = windowManager.getWindow('Finder');
      const initialPos = await windowManager.getWindowPosition(finder);

      await windowManager.dragWindow(finder, 100, 50);

      const newPos = await windowManager.getWindowPosition(finder);
      expect(newPos!.x).toBeGreaterThan(initialPos!.x);
      expect(newPos!.y).toBeGreaterThan(initialPos!.y);
    });

    test('should bring window to front when dragging', async () => {
      const notes = windowManager.getWindow('Notes');
      const finder = windowManager.getWindow('Finder');

      await windowManager.dragWindow(finder, 10, 10);

      const finderZ = await windowManager.getZIndex(finder);
      const notesZ = await windowManager.getZIndex(notes);
      expect(finderZ).toBeGreaterThanOrEqual(notesZ);
    });
  });

  test.describe('Window Resizing', () => {
    test('should resize from east handle', async () => {
      const finder = windowManager.getWindow('Finder');
      const initialPos = await windowManager.getWindowPosition(finder);

      await windowManager.resizeWindow(finder, 'e', 50, 0);

      const newPos = await windowManager.getWindowPosition(finder);
      expect(newPos!.width).toBeGreaterThan(initialPos!.width);
    });

    test('should resize from south handle', async () => {
      const finder = windowManager.getWindow('Finder');
      const initialPos = await windowManager.getWindowPosition(finder);

      await windowManager.resizeWindow(finder, 's', 0, 50);

      const newPos = await windowManager.getWindowPosition(finder);
      expect(newPos!.height).toBeGreaterThan(initialPos!.height);
    });

    test('should resize from southeast corner', async () => {
      const finder = windowManager.getWindow('Finder');
      const initialPos = await windowManager.getWindowPosition(finder);

      await windowManager.resizeWindow(finder, 'se', 50, 50);

      const newPos = await windowManager.getWindowPosition(finder);
      expect(newPos!.width).toBeGreaterThan(initialPos!.width);
      expect(newPos!.height).toBeGreaterThan(initialPos!.height);
    });

    test('should have all 8 resize handles', async () => {
      const handles = ['n', 's', 'e', 'w', 'ne', 'nw', 'se', 'sw'];
      const finder = windowManager.getWindow('Finder');

      for (const handle of handles) {
        const resizeHandle = windowManager.getResizeHandle(finder, handle);
        await expect(resizeHandle).toBeAttached();
      }
    });

    test('should enforce minimum width', async () => {
      const finder = windowManager.getWindow('Finder');

      await windowManager.resizeWindow(finder, 'e', -1000, 0);

      const newPos = await windowManager.getWindowPosition(finder);
      expect(newPos!.width).toBeGreaterThanOrEqual(200);
    });

    test('should enforce minimum height', async () => {
      const finder = windowManager.getWindow('Finder');

      await windowManager.resizeWindow(finder, 's', 0, -1000);

      const newPos = await windowManager.getWindowPosition(finder);
      expect(newPos!.height).toBeGreaterThanOrEqual(100);
    });
  });

  test.describe('Traffic Light Buttons', () => {
    test('should display close button (red)', async () => {
      const finder = windowManager.getWindow('Finder');
      const closeBtn = windowManager.getCloseButton(finder);
      await expect(closeBtn).toBeVisible();
      await expect(closeBtn).toHaveClass(/close/);
    });

    test('should display minimize button (yellow)', async () => {
      const finder = windowManager.getWindow('Finder');
      const minBtn = windowManager.getMinimizeButton(finder);
      await expect(minBtn).toBeVisible();
      await expect(minBtn).toHaveClass(/minimize/);
    });

    test('should display maximize button (green)', async () => {
      const finder = windowManager.getWindow('Finder');
      const maxBtn = windowManager.getMaximizeButton(finder);
      await expect(maxBtn).toBeVisible();
      await expect(maxBtn).toHaveClass(/maximize/);
    });

    test('should close window on close button click', async () => {
      const notes = windowManager.getWindow('Notes');
      await windowManager.getCloseButton(notes).click();

      await expect(notes).not.toBeVisible();
    });

    test('should minimize window on minimize button click', async () => {
      const finder = windowManager.getWindow('Finder');
      await windowManager.getMinimizeButton(finder).click();

      await expect(finder).toHaveClass(/minimized/);
    });

    test('should maximize window on maximize button click', async () => {
      const finder = windowManager.getWindow('Finder');
      await windowManager.getMaximizeButton(finder).click();

      await expect(finder).toHaveClass(/maximized/);
    });

    test('should restore window on second maximize click', async () => {
      const finder = windowManager.getWindow('Finder');

      await windowManager.getMaximizeButton(finder).click();
      await expect(finder).toHaveClass(/maximized/);

      await windowManager.getMaximizeButton(finder).click();
      await expect(finder).not.toHaveClass(/maximized/);
    });
  });

  test.describe('Z-Index Layering', () => {
    test('should bring clicked window to front', async () => {
      const finder = windowManager.getWindow('Finder');
      const calculator = windowManager.getWindow('Calculator');

      await calculator.click();

      const calcZ = await windowManager.getZIndex(calculator);
      const finderZ = await windowManager.getZIndex(finder);

      expect(calcZ).toBeGreaterThan(finderZ);
    });

    test('should set active class on front window', async () => {
      const finder = windowManager.getWindow('Finder');
      const calculator = windowManager.getWindow('Calculator');

      await calculator.click();
      await expect(calculator).toHaveClass(/active/);

      await finder.click();
      await expect(finder).toHaveClass(/active/);
      await expect(calculator).not.toHaveClass(/active/);
    });
  });
});
