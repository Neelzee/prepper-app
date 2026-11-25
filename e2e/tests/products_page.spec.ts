import { test, expect } from '@playwright/test';

test("can create new product", async ({ page }) => {
  await page.goto("/product/new");

  await expect(page.getByLabel("Produkt")).toBeVisible();
});
