import unittest
from sandbox import AiSandbox, SandboxError, ToolRequest


class SandboxTests(unittest.TestCase):
    def test_allows_only_declared_tools(self):
        box = AiSandbox({"search"})
        self.assertEqual(box.run(ToolRequest("search", {"q": "rust"}), lambda _: "ok"), "ok")
        with self.assertRaises(SandboxError):
            box.run(ToolRequest("shell", {}), lambda _: "bad")

    def test_limits_steps(self):
        box = AiSandbox({"echo"}, max_steps=1)
        box.run(ToolRequest("echo", {}), lambda _: None)
        with self.assertRaises(SandboxError):
            box.run(ToolRequest("echo", {}), lambda _: None)


if __name__ == "__main__":
    unittest.main()
