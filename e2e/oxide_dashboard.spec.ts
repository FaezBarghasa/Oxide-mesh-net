import { test, expect } from '@playwright/test';

test.describe('Oxide Mesh Net Dashboard & Navigation E2E', () => {
  test('renders header telemetry and mesh branding', async ({ page }) => {
    await page.goto('/');

    // Check brand banner
    await expect(page.locator('.brand-section')).toContainText('OXIDE MESH NET');
    await expect(page.locator('.brand-section')).toContainText('v0.1.0-pure-rust');

    // Check daemon status telemetry
    await expect(page.locator('.header-telemetry')).toContainText('DAEMON SYNCED');
    await expect(page.locator('.header-telemetry')).toContainText('100.64.0.42');
  });

  test('navigates through desktop sidebar views', async ({ page, isMobile }) => {
    test.skip(isMobile, 'Sidebar test applies to desktop viewports only');
    await page.goto('/');

    // Verify default view is Dashboard
    await expect(page.locator('.nav-item.active')).toContainText('Dashboard');

    // Navigate to Routing & Egress
    await page.click('text=Routing & Egress');
    await expect(page.locator('.nav-item.active')).toContainText('Routing & Egress');

    // Navigate to Stealth & Anti-DPI
    await page.click('text=Stealth & Anti-DPI');
    await expect(page.locator('.nav-item.active')).toContainText('Stealth & Anti-DPI');

    // Navigate to Zero-Trust & ACL
    await page.click('text=Zero-Trust & ACL');
    await expect(page.locator('.nav-item.active')).toContainText('Zero-Trust & ACL');

    // Navigate to Network Services
    await page.click('text=Network Services');
    await expect(page.locator('.nav-item.active')).toContainText('Network Services');

    // Navigate to Merkle Ledger & Debug
    await page.click('text=Merkle Ledger & Debug');
    await expect(page.locator('.nav-item.active')).toContainText('Merkle Ledger & Debug');

    // Navigate to Platform & Adapters
    await page.click('text=Platform & Adapters');
    await expect(page.locator('.nav-item.active')).toContainText('Platform & Adapters');
  });

  test('displays mobile bottom navigation bar on mobile viewport', async ({ page, isMobile }) => {
    test.skip(!isMobile, 'Mobile bottom bar applies to mobile viewports only');
    await page.goto('/');

    // Mobile nav bar should be visible
    const mobileNav = page.locator('.mobile-nav');
    await expect(mobileNav).toBeVisible();

    // Verify mobile nav tabs
    await expect(mobileNav.locator('.mobile-nav-item.active')).toContainText('Status');
    await mobileNav.locator('text=Routing').click();
    await expect(mobileNav.locator('.mobile-nav-item.active')).toContainText('Routing');
  });
});
