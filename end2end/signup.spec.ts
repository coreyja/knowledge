import { test, expect } from "./setup";

test("can create account and then login", async ({ page }) => {
  await page.goto("/signup");

  const username = "test" + Math.floor(Math.random() * 100000);

  await page.getByRole("textbox", { name: "Username" }).fill(username);
  await page.getByRole("textbox", { name: "Password" }).fill("password");

  await page.getByRole("button", { name: "Signup" }).click();

  await expect(page.getByText(`Welcome, ${username}`)).toBeVisible();

  await page.goto("/signout");

  await expect(page.getByText(`Welcome, ${username}`)).not.toBeVisible();

  await page.goto("/login");

  await page.getByRole("textbox", { name: "Username" }).fill(username);
  await page.getByRole("textbox", { name: "Password" }).fill("password");

  await page.getByRole("button", { name: "Login" }).click();

  await expect(page.getByText(`Welcome, ${username}`)).toBeVisible();
});
