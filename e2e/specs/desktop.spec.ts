import { test, expect } from '@playwright/test';
import { DesktopPage } from '../page-objects';

test.describe('Desktop', () => {
  let desktop: DesktopPage;

  test.beforeEach(async ({ page }) => {
    desktop = new DesktopPage(page);
    await desktop.goto();
  });

});
