import { test as base } from "@playwright/test";
import { SignupPage } from "./pages/Signup";

type Fixtures = {
  loggedIn: SignupPage;
};

const test = base.extend<Fixtures>({
  loggedIn: async ({ page }, use) => {
    const signupPage = new SignupPage(page);
    await signupPage.createUser();
    await use(signupPage);
  },
});

export { test };
export { expect } from "@playwright/test";
