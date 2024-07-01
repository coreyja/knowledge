import { test as base } from "@playwright/test";
// import { TodoPage } from "./todo-page";

// Extend basic test by providing a "todoPage" fixture.
const test = base.extend<{ todoPage: TodoPage }>({
  todoPage: async ({ page }, use) => {
    const todoPage = new TodoPage(page);
    await todoPage.goto();
    await todoPage.addToDo("item1");
    await todoPage.addToDo("item2");
    await use(todoPage);
    await todoPage.removeAll();
  },
});
