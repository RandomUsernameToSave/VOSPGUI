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


// Ensure that 'Z_charge' is passed as an object property with a key corresponding to the Rust function's parameter name.
import { invoke } from '@tauri-apps/api/core';

let Z_charge = 0;
let element_name = "electron";
let lv = 5. ; 
let mu = 1. ;
let init_cond = "zero";
let right_boundary = "";
let left_boundary = "";
// Make sure 'z_charge' key exactly matches the parameter in Rust
$: console.log(init_cond);

let IsClicked = false;

function addElement() {
  IsClicked= true;
  invoke('new_element', {elementName:element_name,lv:lv,initCond:init_cond
    ,zCharge:Z_charge,mu:mu,rightBoundary:right_boundary,leftBoundary:left_boundary })
  console.log(element_name);
  console.log(lv);
  toast("Successfuly added : "+element_name);
  
  setTimeout(() => {
    IsClicked = false;
		}, 1000);
  
}
</script>




<main class="grid h-dvh flex-1 gap-4 overflow-auto p-4 md:grid-cols-2 lg:grid-cols-3">


    <div
        class="bg-muted/50 relative flex h-full min-h-[50vh] flex-col rounded-xl p-4 lg:col-span-2"
    >
    <Card.Root>
    <Card.Header class="px-7">
        <Card.Title>Add an Element</Card.Title>
        <Card.Description>A list of the elements in the solver.</Card.Description>
    </Card.Header>
    <Card.Content>
        <form class="grid w-full items-start gap-6" method="POST">
            <fieldset class="grid gap-6 rounded-lg border p-4">
              <legend class="-ml-1 px-1 text-sm font-medium"> Element settings </legend>
              <div class="grid gap-3">
                <Label for="model">Element</Label>
                <Select.Root selected={right_boundary} onSelectedChange={(v) => {
                  v && (right_boundary = parseInt(v.value));
                }}>
                  <Select.Trigger
                    id="model"
                    class="items-start [&_[data-description]]:hidden"
                  >
                    <Select.Value placeholder="Select species right boundary condition" />
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
                    <Select.Item value="periodic">
                      <div class="text-muted-foreground flex items-start gap-3">
                        <Bird class="size-5" />
                        <div class="grid gap-0.5">
                          <p>
                            <span class="text-foreground font-medium">
                              Periodic
                            </span>
                          </p>
      
                        </div>
                      </div>
                    </Select.Item>
                  </Select.Content>
                </Select.Root>
                <Select.Root selected={left_boundary} onSelectedChange={(v) => {
                  v && (left_boundary = parseInt(v.value));
                }}>
                  <Select.Trigger
                    id="model"
                    class="items-start [&_[data-description]]:hidden"
                  >
                    <Select.Value placeholder="Select species left boundary condition" />
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
                    <Select.Item value="periodic">
                      <div class="text-muted-foreground flex items-start gap-3">
                        <Bird class="size-5" />
                        <div class="grid gap-0.5">
                          <p>
                            <span class="text-foreground font-medium">
                              Periodic
                            </span>
                          </p>
      
                        </div>
                      </div>
                    </Select.Item>
                  </Select.Content>
                </Select.Root>
              </div>
              <Select.Root selected={init_cond} onSelectedChange={(v) => {
                v && (init_cond = parseInt(v.value));
              }}> <!-- Two-way binding between Select and init_cond -->
                <Select.Trigger id="model" class="items-start [&_[data-description]]:hidden">
                  <Select.Value placeholder="Select initial boundary condition" />
                </Select.Trigger>
              
                <Select.Content >
                  <Select.Item value="maxwellian">
                    <div class="text-muted-foreground flex items-start gap-3">
                      <Rabbit class="size-5" />
                      <div class="grid gap-0.5">
                        <p>
                          <span class="text-foreground font-medium">Maxwellian</span>
                        </p>
                      </div>
                    </div>
                  </Select.Item>
              
                  <Select.Item value="zero">
                    <div class="text-muted-foreground flex items-start gap-3">
                      <Bird class="size-5" />
                      <div class="grid gap-0.5">
                        <p>
                          <span class="text-foreground font-medium">Zero</span>
                        </p>
                      </div>
                    </div>
                  </Select.Item>
              
                  <Select.Item value="half-maxwellian">
                    <div class="text-muted-foreground flex items-start gap-3">
                      <Bird class="size-5" />
                      <div class="grid gap-0.5">
                        <p>
                          <span class="text-foreground font-medium">Maxwellian then 0 at half the spatial domain size</span>
                        </p>
                      </div>
                    </div>
                  </Select.Item>
                </Select.Content>
              </Select.Root>
              <div class="grid grid-cols-2 gap-4">
                <div class="grid gap-3">
                  <Label for="top-p">Element name</Label>
                  <Input id="top-p" type="text" bind:value={element_name} placeholder="electron" />
                </div>
                <div class="grid gap-3">
                  <Label for="top-k">Length velocity direction</Label>
                  <Input id="top-k" type="number" bind:value={lv} placeholder="30." />
                </div>
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div class="grid gap-3">
                  <Label for="top-p">Z charge</Label>
                  <Input id="top-p" type="number" bind:value={
                    () => Z_charge,
                    (v) => Z_charge = Math.round(v)} placeholder="-1" />
                </div>
                <div class="grid gap-3">
                  <Label for="top-k">Ratio of mass with an electron</Label>
                  <Input id="top-k" type="number" bind:value={mu} placeholder="1." />
                </div>
              </div>

              <Button on:click="{addElement}" disabled={IsClicked}>
                {#if IsClicked}
                  <span>Element added!</span>
                {:else}
                  <span>Add element</span>
                {/if}
              </Button>
            </fieldset>
          </form>
        </Card.Content>
    </Card.Root>



    </div>
</main>
