<script lang="ts">
    import { page } from '$app/stores';  // Import the SvelteKit page store
    import Triangle from "lucide-svelte/icons/triangle";
    import Atom from "lucide-svelte/icons/atom";
    import SquareTerminal from "lucide-svelte/icons/square-terminal";
    import ChartColumn from "lucide-svelte/icons/chart-column";
    import LifeBuoy from "lucide-svelte/icons/life-buoy";
    import Book from "lucide-svelte/icons/book";
    import SquareUser from "lucide-svelte/icons/square-user";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    let currentPage;
    $: currentPage = $page.url.pathname;
</script>
  
  <aside class="inset-y fixed left-0 z- flex h-full flex-col border-r">
    <div class="border-b p-2">
      <Button variant="outline" size="icon" aria-label="Home">
        <span class="font-bold">V</span>
      </Button>
    </div>
    <nav class="grid gap-1 p-2">
      <Tooltip.Root>
        <Tooltip.Trigger asChild let:builder>
          <a href="/">
            <Button variant="ghost" size="icon" class="{currentPage === '/' ? 'bg-secondary' : ''} rounded-lg" aria-label="Playground" builders={[builder]}>
              <SquareTerminal class="size-5" />
            </Button>
          </a>
        </Tooltip.Trigger>
        <Tooltip.Content side="right" sideOffset={5}>Solver</Tooltip.Content>
      </Tooltip.Root>
  
      <Tooltip.Root>
        <Tooltip.Trigger asChild let:builder>
          <a href="/Element">
            <Button variant="ghost" size="icon" class="{currentPage === '/Element' ? 'bg-secondary' : ''} rounded-lg" aria-label="Models" builders={[builder]}>
              <Atom class="size-5" />
            </Button>
          </a>
        </Tooltip.Trigger>
        <Tooltip.Content side="right" sideOffset={5}>Elements</Tooltip.Content>
      </Tooltip.Root>
  
      <Tooltip.Root>
        <Tooltip.Trigger asChild let:builder>
          <a href="/Data">
            <Button variant="ghost" size="icon" class="{currentPage === '/Data' ? 'bg-secondary' : ''} rounded-lg" aria-label="Models" builders={[builder]}>
              <ChartColumn class="size-5" />
            </Button>
          </a>
        </Tooltip.Trigger>
        <Tooltip.Content side="right" sideOffset={5}>Data</Tooltip.Content>
      </Tooltip.Root>
  
      <Tooltip.Root>
        <Tooltip.Trigger asChild let:builder>
          <Button variant="ghost" size="icon" class="rounded-lg" aria-label="Documentation" builders={[builder]}>
            <Book class="size-5" />
          </Button>
        </Tooltip.Trigger>
        <Tooltip.Content side="right" sideOffset={5}>Documentation</Tooltip.Content>
      </Tooltip.Root>
    </nav>
  
    <nav class="mt-auto grid gap-1 p-2">
      <Tooltip.Root>
        <Tooltip.Trigger asChild let:builder>
          <Button variant="ghost" size="icon" class="mt-auto rounded-lg" aria-label="Help" builders={[builder]}>
            <LifeBuoy class="size-5" />
          </Button>
        </Tooltip.Trigger>
        <Tooltip.Content side="right" sideOffset={5}>Help</Tooltip.Content>
      </Tooltip.Root>
  
      <Tooltip.Root>
        <Tooltip.Trigger asChild let:builder>
          <Button variant="ghost" size="icon" class="mt-auto rounded-lg" aria-label="Account" builders={[builder]}>
            <SquareUser class="size-5" />
          </Button>
        </Tooltip.Trigger>
        <Tooltip.Content side="right" sideOffset={5}>Account</Tooltip.Content>
      </Tooltip.Root>
    </nav>
  </aside>
  