import { Router } from "https://deno.land/x/oak/mod.ts";

const router = new Router();

import todoController from "../controllers/todo.ts";

router
  .get("/todos", getDinosaurs)
  .post("/todos", getDinosaur)
  .get("/todos/:id", addDinosaur)
  .put("/todos/:id", addDinosaur)
  .delete("/todos/:id", deleteDinosaur);

export default router;
