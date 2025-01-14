<script lang="ts">
	import Navbar from '$lib/components/Navbar.svelte';
	import "../app.css";

  import { Toaster } from "$lib/components/ui/sonner";
</script>

 
<Toaster />
  
  <div class="grid h-screen w-full pl-[53px] overflow-hidden">
	<!-- Import the Navbar component here -->
	<Navbar />
  
	<div class="flex flex-col">
	  <main class="flex-1 overflow-auto p-4">
		<slot /> <!-- Content will be displayed here -->
	  </main>
	</div>
  </div>
  