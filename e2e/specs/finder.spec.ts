import { test, expect } from '@playwright/test';
import { FinderPage, WindowManagerPage, DesktopPage } from '../page-objects';

test.describe('Finder', () => {
  let finder: FinderPage;
  let windowManager: WindowManagerPage;

  test.beforeEach(async ({ page }) => {
    const desktop = new DesktopPage(page);
    await desktop.goto();
    windowManager = new WindowManagerPage(page);
    const finderWindow = windowManager.getWindow('Finder');
    finder = new FinderPage(page, finderWindow);
  });

  test.describe('Sidebar Navigation', () => {
    test('should display Favorites section', async () => {
      await expect(finder.sidebar.locator('.sidebar-header', { hasText: 'Favorites' })).toBeVisible();
    });

    test('should display Locations section', async () => {
      await expect(finder.sidebar.locator('.sidebar-header', { hasText: 'Locations' })).toBeVisible();
    });

    test('should show AirDrop in sidebar', async () => {
      await expect(finder.getSidebarItem('AirDrop')).toBeVisible();
    });

    test('should show Recents in sidebar', async () => {
      await expect(finder.getSidebarItem('Recents')).toBeVisible();
    });

    test('should show Applications in sidebar', async () => {
      await expect(finder.getSidebarItem('Applications')).toBeVisible();
    });

    test('should show Desktop in sidebar', async () => {
      await expect(finder.getSidebarItem('Desktop')).toBeVisible();
    });

    test('should show Documents in sidebar', async () => {
      await expect(finder.getSidebarItem('Documents')).toBeVisible();
    });

    test('should show Downloads in sidebar', async () => {
      await expect(finder.getSidebarItem('Downloads')).toBeVisible();
    });

    test('should show Macintosh HD in sidebar', async () => {
      await expect(finder.getSidebarItem('Macintosh HD')).toBeVisible();
    });

    test('should navigate to Applications on click', async () => {
      await finder.navigateToSidebarItem('Applications');

      await expect(finder.getSidebarItem('Applications')).toHaveClass(/selected/);
    });

    test('should navigate to Desktop on click', async () => {
      await finder.navigateToSidebarItem('Desktop');

      await expect(finder.getSidebarItem('Desktop')).toHaveClass(/selected/);
    });

    test('should navigate to Documents on click', async () => {
      await finder.navigateToSidebarItem('Documents');

      await expect(finder.getSidebarItem('Documents')).toHaveClass(/selected/);
    });

    test('should navigate to Downloads on click', async () => {
      await finder.navigateToSidebarItem('Downloads');

      await expect(finder.getSidebarItem('Downloads')).toHaveClass(/selected/);
    });
  });

  test.describe('File Grid Display', () => {
    test('should display files in content area', async () => {
      const fileCount = await finder.getFileCount();
      expect(fileCount).toBeGreaterThan(0);
    });

    test('should display file grid', async () => {
      await expect(finder.fileGrid).toBeVisible();
    });
  });

  test.describe('File Selection', () => {
    test('should select file on click', async () => {
      const files = finder.fileGrid.locator('.finder-item');
      const firstFile = files.first();

      await firstFile.click();

      await expect(firstFile).toHaveClass(/selected/);
    });

    test('should toggle selection on second click', async () => {
      const files = finder.fileGrid.locator('.finder-item');
      const firstFile = files.first();

      await firstFile.click();
      await expect(firstFile).toHaveClass(/selected/);

      await firstFile.click();
      await expect(firstFile).not.toHaveClass(/selected/);
    });
  });

  test.describe('View Modes', () => {
    test('should display view mode buttons', async () => {
      const count = await finder.viewButtons.count();
      expect(count).toBe(4);
    });

    test('should have first view button as active by default', async () => {
      const isActive = await finder.isViewButtonActive(0);
      expect(isActive).toBe(true);
    });
  });

  test.describe('Toolbar', () => {
    test('should display back navigation button', async () => {
      await expect(finder.navBackButton).toBeVisible();
    });

    test('should display forward navigation button', async () => {
      await expect(finder.navForwardButton).toBeVisible();
    });

    test('should display search input', async () => {
      await expect(finder.searchInput).toBeVisible();
    });

    test('should have search placeholder text', async () => {
      await expect(finder.searchInput).toHaveAttribute('placeholder', 'Search');
    });
  });

  test.describe('Status Bar', () => {
    test('should display item count', async () => {
      const statusText = await finder.getStatusBarText();
      expect(statusText).toMatch(/\d+ items?/);
    });
  });
});
