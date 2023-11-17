<script lang="ts">
    import { onMount } from "svelte";
  
    // ======== APPLICATION STATE ========
  
    let wallet: any;
    let account = "";
  
    // reactively log the wallet connection when account state changes,
    // if you don't know what this is, check out https://svelte.dev/tutorial/reactive-declarations
    $: account && console.log(`Connected to wallet: ${account}`);
  
    // life cycle hook for when the component is mounted
    onMount(() => {

    const { solana } = window as any;
      wallet = solana;
  
      // set up handlers for wallet events
      wallet.on("connect", () => (account = wallet.publicKey.toString()));
      wallet.on("disconnect", () => (account = ""));  

      (async() => {await wallet.connect({ onlyIfTrusted: true })})()

    });
  
    // ======== CONNECT WALLET ========
    const handleConnectWallet = async () => {
      const resp = await wallet.connect();
    };
  </script>
  
  <main>
    <h1>gm, Solana!</h1>
  
    <!-- Conditionally render the user account, connect button, or just a warning -->
    {#if account}
    <h3>Your wallet:</h3>
    <p>{account}</p>
    {:else if wallet} {#if wallet.isPhantom}
    <h2>Phantom Wallet found!</h2>
    <button on:click="{handleConnectWallet}">Connect wallet</button>
    {:else}
    <h2>Solana wallet found but not supported.</h2>
    {/if} {:else}
    <h2>Solana wallet not found.</h2>
    {/if}
  </main>
  
  <style>
    main {
      text-align: center;
      padding: 1em;
      max-width: 240px;
      margin: 0 auto;
    }
  
    h1 {
      color: #ff3e00;
      font-size: 4em;
      font-weight: 100;
    }
  
    @media (min-width: 640px) {
      main {
        max-width: none;
      }
    }
  </style>