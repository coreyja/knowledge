import { test, expect } from "./setup";

test.skip("can add article and get a summary", async ({ loggedIn, page }) => {
  await page.goto("/dashboard");

  await page
    .getByRole("textbox", { name: "URL :" })
    .fill("example.org/some-article");

  await page.getByRole("button", { name: "Submit" }).click();

  await expect(page.getByText("Generating snapshot.....")).toBeVisible();

  let attempts = 0;
  while (attempts < 100) {
    if (
      await page
        .getByText("Hello there, how may I assist you today?")
        .isVisible()
    ) {
      break;
    }
    await new Promise((r) => setTimeout(r, 1000));
    await page.reload();
    attempts++;
  }

  await expect(
    page.getByText("Hello there, how may I assist you today?")
  ).toBeVisible();
});
