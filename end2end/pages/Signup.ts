import { type Page, type Locator, expect } from "@playwright/test";

export class SignupPage {
  constructor(public readonly page: Page) {}

  async createUser() {
    await this.page.goto("/signup");

    const username = "test" + Math.floor(Math.random() * 100000);
    const password = "password";

    await this.page.getByRole("textbox", { name: "Username" }).fill(username);
    await this.page.getByRole("textbox", { name: "Password" }).fill(password);

    await this.page.getByRole("button", { name: "Signup" }).click();

    await expect(this.page.getByText(`Welcome, ${username}`)).toBeVisible();

    return { username, password };
  }
}
