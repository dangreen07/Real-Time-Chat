import { Moon, Sun } from "lucide-react"
import { Theme, useTheme } from "remix-themes"
import { Button } from "./ui/button"

export function ModeToggle() {
  const [theme, setTheme] = useTheme()

  return (
    <Button size="icon" onClick={() => {
      if (theme === Theme.DARK) {
        setTheme(Theme.LIGHT)
      }
      else {
        setTheme(Theme.DARK)
      }
    }}>
      { theme === Theme.DARK ? <Sun className="h[1.2rem] w-[1.2rem]" /> : <Moon className="h[1.2rem] w-[1.2rem]" /> }
      <span className="sr-only">Toggle theme</span>
    </Button>
  )
}