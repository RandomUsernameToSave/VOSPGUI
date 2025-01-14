<script lang="ts">
  import Triangle from "lucide-svelte/icons/triangle";
  import Atom from "lucide-svelte/icons/atom";
  import SquareTerminal from "lucide-svelte/icons/square-terminal";
  import ChartColumn  from 'lucide-svelte/icons/chart-column';
  import LifeBuoy from "lucide-svelte/icons/life-buoy";
  import Book from "lucide-svelte/icons/book";
  import SquareUser from "lucide-svelte/icons/square-user";
  import Settings from "lucide-svelte/icons/settings";
  import Rabbit from "lucide-svelte/icons/rabbit";
  import Bird from "lucide-svelte/icons/bird";
  import Paperclip from "lucide-svelte/icons/paperclip";
  import Mic from "lucide-svelte/icons/mic";
  import CornerDownLeft from "lucide-svelte/icons/corner-down-left";

  import { Progress } from "$lib/components/ui/progress";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import * as Drawer from "$lib/components/ui/drawer/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import * as Card from "$lib/components/ui/card";
  import { toast } from "svelte-sonner";
  import * as Table from "$lib/components/ui/table/index.js";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from '@tauri-apps/api/event';


  let elements ;
  let DT=0.000005;
  let end_time=0.001;;
  let NX= 100;
  let NV = 100;
  let LX = 1.
  let lambda = 0.01;
  let potentialRightBoundary = "zero";
  let potentialLeftBoundary = "zero";
  let IsClicked = false;
  let nSave = 100;
  let value = 0;
  
  invoke("get_elements").then((element_list)=> {
    elements = element_list;
    console.log(elements);
  })

  function RunSolver() {
    invoke("run_solver");
  }

  function addConfig() {
    IsClicked= true;
    invoke("addConfig", {nx:NX,nv:NV,lx:LX,dt:DT,endTime:end_time,lambda:lambda,fieldBcLeft:potentialRightBoundary,fieldBcRight:potentialLeftBoundary,nSave:nSave});

    toast("Successfuly added configuration to Solver");
    setTimeout(() => {
    IsClicked = false;
		}, 1000);
  }

  listen<number>('progress-update', (event) => {
    value = event.payload;
    if (value==99) {
      value = 0;
    }
    console.log(value);
  });
</script>





<main class="h-dvh grid flex-1 gap-4 overflow-auto p-4 md:grid-cols-2 lg:grid-cols-3">
  <div class="relative hidden flex-col items-start gap-8 md:flex">
    <form class="grid w-full items-start gap-6">
      <fieldset class="grid gap-6 rounded-lg border p-4">
        <legend class="-ml-1 px-1 text-sm font-medium"> Solver settings </legend>
        <div class="grid gap-3">
          <Label for="model">Potential Field</Label>
            <Select.Root selected={potentialRightBoundary} onSelectedChange={(v) => {
                v && (potentialRightBoundary = parseInt(v.value));
              }}>
            <Select.Trigger
              id="model"
              class="items-start [&_[data-description]]:hidden"
            >
              <Select.Value placeholder="Select Potential right boundary condition" />
            </Select.Trigger>
            <Select.Content>
              <Select.Item value="zero">
                <div class="text-muted-foreground flex items-start gap-3">
                  <Rabbit class="size-5" />
                  <div class="grid gap-0.5">
                    <p>
                      <span class="text-foreground font-medium">
                        Dirichlet
                      </span>
                    </p>
                  </div>
                </div>
              </Select.Item>
              <Select.Item value="neumann">
                <div class="text-muted-foreground flex items-start gap-3">
                  <Bird class="size-5" />
                  <div class="grid gap-0.5">
                    <p>
                      <span class="text-foreground font-medium">
                        Neumann
                      </span>
                    </p>

                  </div>
                </div>
              </Select.Item>
            </Select.Content>
          </Select.Root>
          <Select.Root selected={potentialLeftBoundary} onSelectedChange={(v) => {
            v && (potentialLeftBoundary = parseInt(v.value));
          }}>
            <Select.Trigger
              id="model"
              class="items-start [&_[data-description]]:hidden"
            >
              <Select.Value placeholder="Select Potential left boundary condition" />
            </Select.Trigger>
            <Select.Content>
              <Select.Item value="zero">
                <div class="text-muted-foreground flex items-start gap-3">
                  <Rabbit class="size-5" />
                  <div class="grid gap-0.5">
                    <p>
                      <span class="text-foreground font-medium">
                        Dirichlet
                      </span>
                    </p>
                  </div>
                </div>
              </Select.Item>
              <Select.Item value="neumann">
                <div class="text-muted-foreground flex items-start gap-3">
                  <Bird class="size-5" />
                  <div class="grid gap-0.5">
                    <p>
                      <span class="text-foreground font-medium">
                        Neumann
                      </span>
                    </p>

                  </div>
                </div>
              </Select.Item>
            </Select.Content>
          </Select.Root>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div class="grid gap-3">
            <Label for="top-p">DT</Label>
            <Input id="top-p" type="number" placeholder="0.000001" bind:value={DT} />
          </div>
          <div class="grid gap-3">
            <Label for="top-k">Final Time</Label>
            <Input id="top-k" type="number" placeholder="0.0001" bind:value={end_time}/>
          </div>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div class="grid gap-3">
            <Label for="top-p">NX</Label>
            <Input id="top-p" type="number" placeholder="0.7" bind:value={NX} />
          </div>
          <div class="grid gap-3">
            <Label for="top-k">NV</Label>
            <Input id="top-k" type="number" placeholder="0.0" bind:value={NV}/>
          </div>
        </div>
        <div class="grid gap-3">
          <Label for="top-p">Length of simulation (LX) </Label>
          <Input id="LX" type="number" placeholder="1." bind:value={LX} />
        </div>
        <div class="grid gap-3">
          <Label for="top-p">Lambda </Label>
          <Input id="LX" type="number" placeholder="0.1" bind:value={lambda} />
        </div>
        <div class="grid gap-3">
          <Label for="top-p">Save every N iteration </Label>
          <Input id="nsave" type="number" placeholder="0.1" bind:value={nSave} />
        </div>
        <Button on:click="{addConfig}" disabled={IsClicked}>
          {#if IsClicked}
            <span>Config saved!</span>
          {:else}
            <span>Save config</span>
          {/if}
        </Button>
      </fieldset>
    </form>
  </div>


  <div
    class="bg-muted/50 relative flex h-full min-h-[50vh] flex-col rounded-xl p-4 lg:col-span-2"
  >
  <Card.Root>
  <Card.Header class="px-7">
    <Card.Title>Elements List</Card.Title>
    <Card.Description>A list of the elements in the solver.</Card.Description>
  </Card.Header>
  <Card.Content>
  <Table.Root>

    <Table.Header>
      <Table.Row>
        <Table.Head class="w-[100px]">Element name</Table.Head>
        <Table.Head>Mass</Table.Head>
        <Table.Head>Charge</Table.Head>
        <Table.Head class="text-right">lv</Table.Head>
      </Table.Row>
    </Table.Header>
    <Table.Body>
      {#each elements as element, i (i)}
        <Table.Row>
          <Table.Cell class="font-medium">{element.element_name}</Table.Cell>
          <Table.Cell>{element.mu}</Table.Cell>
          <Table.Cell>{element.z_charge}</Table.Cell>
          <Table.Cell class="text-right">{element.lv}</Table.Cell>
        </Table.Row>
      {/each}
    </Table.Body>
  </Table.Root>
    </Card.Content>
  </Card.Root>
    <div class="flex-1" />
    <div class="w-32 absolute bottom-3 right-3">
      <Button on:click="{RunSolver}" >Start Solver ></Button>
    </div>
    <Progress {value} max={100} class="w-[60%]" /> 
    <p>{value} %</p>

  </div>
</main>


