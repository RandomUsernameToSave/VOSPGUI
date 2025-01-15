<script lang="ts">
  import { onMount } from "svelte";
  import * as echarts from "echarts";
  import * as Card from "$lib/components/ui/card";
  import * as Table from "$lib/components/ui/table/index.js";
  import { Checkbox } from "$lib/components/ui/checkbox/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { invoke } from "@tauri-apps/api/core";


  function reshapeData(data: number[][]): number[][] {
  const reshapedData = [];
  for (let i = 0; i < data.length; i++) {
    for (let j = 0; j < data[i].length; j++) {
      reshapedData.push([i, j, data[i][j]]);
    }
  }
  return reshapedData;
  }



  let checked = false;
  let chartData = [[0]];
  let chartElement;
  let chart;

  // Generate xData and yData from the chartData dimensions
  let xData = Array.from({ length: chartData.length }, (v, i) => (i + 1)/100);
  let yData = Array.from({ length: chartData[0].length }, (v, i) => (i + 1)/100);

  // Initialize the chart once the component is mounted
  onMount(() => {
    chart = echarts.init(chartElement);

    // Set the initial chart options
    let option = {
      tooltip: {},
      xAxis: {
        type: 'category',
        data: xData
      },
      yAxis: {
        type: 'category',
        data: yData
      },
      visualMap: {
        min: 0,
        max: 1,
        calculable: true,
        realtime: false,
        inRange: {
          color: [
            '#313695',
            '#4575b4',
            '#74add1',
            '#abd9e9',
            '#e0f3f8',
            '#ffffbf',
            '#fee090',
            '#fdae61',
            '#f46d43',
            '#d73027',
            '#a50026'
          ]
        }
      },
      series: [
        {
          name: 'Gaussian',
          type: 'heatmap',
          data: chartData,
          emphasis: {
            itemStyle: {
              borderColor: '#333',
              borderWidth: 1
            }
          },
          progressive: 1000,
          animation: false
        }
      ]
    };

    // Set the options to the chart
    chart.setOption(option);

    // Handle window resize for responsive charts
    window.addEventListener("resize", () => {
      chart.resize();
    });
  });

  // Reactive statement to update the chart whenever chartData changes
  $: if (chart) {
    xData = Array.from({ length: chartData.length }, (v, i) => (i + 1)/100);
    yData = Array.from({ length: chartData[0].length }, (v, i) => (i + 1)/100 );

    // Update the chart options with the new data
    chart.setOption({
      xAxis: {
        data: xData
      },
      yAxis: {
        data: yData
      },
      visualMap: {
        max:Math.max(...chartData.flat())
      },

      series: [
        {
          data: reshapeData(chartData)
        }
      ]
    });
  }

  let listed_files = [];
  invoke('list_all_files').then((data) => {
    listed_files = data;
  });

  let group_list = [];
  let selected_file = "";
  $: console.log(selected_file);

  function read_groups() {
    invoke('read_h5', { pathH5: selected_file }).then((data) => {
      group_list = data.filter((item) => item !== "potential field" && item !== "electric field");
      console.log(data);
    });
  }

  let selected_group = '';
  let dataset_list = [];

  function read_dataset() {
    invoke('read_dataset_h5', { pathH5: selected_file, groupH5: selected_group }).then((data) => {
      console.log(data);
      dataset_list = data;
    });
  }

  let selected_dataset = "";

  function select_h5_dataset() {
    invoke('select_h5_dataset', { pathH5: selected_file, groupH5: selected_group, dataset: selected_dataset }).then((data) => {
      chartData = data;  // This will trigger the reactive block to update the chart
    });
    console.log(chartData);
  }
</script>


  <!-- The container for the EChart -->
<div class=" grid grid-flow-col bg-muted p-4 gap-4">

    <Card.Root class="w-[350px]">
        <Card.Header>
          <Card.Title>Create project</Card.Title>
          <Card.Description>Deploy your new project in one-click.</Card.Description>
        </Card.Header>
        <Card.Content>
        
        

        
          <Select.Root selected={selected_file} onSelectedChange={(v) => {
            v && (selected_file = v.value)
            read_groups();
          }}>
            <Select.Trigger class="w-[180px] ">
            <Select.Value placeholder="Select Element" />
            </Select.Trigger>
            <Select.Content>
            <Select.Group >
                <Select.Label>Data</Select.Label>
                {#each listed_files as data}
                <Select.Item value={data} label={data}
                    >{data}</Select.Item
                >
                {/each}
            </Select.Group>
            </Select.Content>
            <Select.Input name="favoriteFruit" />
        </Select.Root>


        <div class="p-4"/>


        <Select.Root selected={selected_group} onSelectedChange={(v) => {
          v && (selected_group = v.value)
          read_dataset();
          }}>
            <Select.Trigger class="w-[180px]">
            <Select.Value placeholder="Select Time" />
            </Select.Trigger>
            <Select.Content>
            <Select.Group>
              <Select.Label>Data</Select.Label>
              {#each group_list as data}
              <Select.Item value={data} label={data}
                  >{data}</Select.Item
              >
              {/each}
            </Select.Group>
            </Select.Content>
            <Select.Input name="favoriteFruit" />
        </Select.Root>

        <Select.Root selected={selected_dataset} onSelectedChange={(v) => {
          v && (selected_dataset = v.value)
          select_h5_dataset();
          }}>
            <Select.Trigger class="w-[180px]">
            <Select.Value placeholder="Select Time" />
            </Select.Trigger>
            <Select.Content>
            <Select.Group>
              <Select.Label>Data</Select.Label>
              {#each dataset_list as data}
              <Select.Item value={data} label={data}
                  >{data}</Select.Item
              >
              {/each}
            </Select.Group>
            </Select.Content>
            <Select.Input name="favoriteFruit" />
        </Select.Root>



    </Card.Content>
  </Card.Root>

  <Card.Root >
    <Card.Header>
      <Card.Title>Heatmap</Card.Title>
      <Card.Description>Vlasov phase-space plot </Card.Description>
    </Card.Header>
    <Card.Content>
      <div bind:this={chartElement} class="w-full h-[400px]"></div>

    </Card.Content>
  </Card.Root>
</div>

  