<script>
  import { create_rune } from "$lib/sdk";
  import { btc_address } from "$lib/store";
  import { toast } from "@zerodevx/svelte-toast";

  let symbol = ""
  let amount = 0
  let decimal = 0

  let tx_hash = null
  let loading = false
</script>

<div class="px-4 sm:px-6 lg:px-8 flex items-center justify-center">
  <!-- Be sure to use this with a layout container that is full-width on mobile -->
  <div
    class="divide-y divide-gray-200 w-1/2 overflow-hidden rounded-lg bg-white shadow"
  >
    <div class="px-4 py-5 sm:px-6">
      <p class="text-center">Launch Rune</p>
    </div>
    <div class="px-4 py-5 sm:p-6">
      <div>
        <label
          for="email"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Your Bitcoin Deposit Address</label
        >
        <div class="mt-2 flex">
          <input
            type="email"
            disabled
            readonly
            name="email"
            bind:value={$btc_address}
            id="email"
            class="block bg-gray-200 w-full rounded-l-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder="you@example.com"
          />
          <button
            on:click={(e) => {
              console.log($btc_address);

              if ($btc_address) {
                navigator.clipboard.writeText($btc_address).then(() => {
                  toast.push("Copied Successfully", {
                    theme: {
                      "--toastColor": "mintcream",
                      "--toastBackground": "rgba(72,187,120,1)",
                      "--toastBarBackground": "#2F855A",
                    },
                  });
                });
              }
            }}
            type="button"
            class="rounded-r bg-white px-2 py-1 text-xs font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-200"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-6 h-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75"
              />
            </svg>
          </button>
        </div>
      </div>

      <div class="mt-4">
        <label
          for="symbol"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Symbol</label
        >
        <div class="mt-2">
          <input
            type="text"
            name="symbol"
            bind:value={symbol}
            id="symbol"
            class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder="you@example.com"
          />
        </div>
      </div>

      <div class="mt-4">
        <label
          for="amount"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Rune Total Supply</label
        >
        <div class="mt-2">
          <input
          bind:value={amount}
            type="number"
            id="amount"
            min="0"
            class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder="450"
          />
        </div>
      </div>

      <div class="mt-4">
        <label
          for="decimal"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Rune Decimal</label
        >
        <div class="mt-2">
          <input
            type="number"
            id="decimal"
            bind:value={decimal}
            min="0"
            class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder="8"
          />
        </div>
      </div>

      {#if tx_hash}
      <div>
        <label
          for="email"
          class="block text-sm font-medium leading-6 text-gray-900"
          >Previous Transaction Id</label
        >
        <div class="mt-2 flex">
          <input
            type="email"
            disabled
            readonly
            name="email"
            bind:value={tx_hash}
            id="email"
            class="block bg-gray-200 w-full rounded-l-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder="you@example.com"
          />
          <button
            on:click={(e) => {
              
              if (tx_hash) {
                navigator.clipboard.writeText(tx_hash).then(() => {
                  toast.push("Copied Successfully", {
                    theme: {
                      "--toastColor": "mintcream",
                      "--toastBackground": "rgba(72,187,120,1)",
                      "--toastBarBackground": "#2F855A",
                    },
                  });
                });
              }
            }}
            type="button"
            class="rounded-r bg-white px-2 py-1 text-xs font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-200"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-6 h-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75"
              />
            </svg>
          </button>
        </div>
      </div>
      {/if}

      <div class="mt-6">
        <button
        disabled={loading}
          on:click={e => {
            loading = true
            create_rune(symbol, BigInt(amount), BigInt(decimal)).then(dt => {
              if (dt.Ok) {
                tx_hash = dt.Ok
                toast.push("Created Successfully", {
                    theme: {
                      "--toastColor": "mintcream",
                      "--toastBackground": "rgba(72,187,120,1)",
                      "--toastBarBackground": "#2F855A",
                    },
                  });
              } else {
                toast.push("Rune Failed", {
                    theme: {
                      "--toastColor": "mintcream",
                      "--toastBackground": "red",
                      "--toastBarBackground": "#2F855A",
                    },
                  });
              }
            }).finally(() => {
              loading = false
            }).catch(err => {
              toast.push("Rune Failed", {
                    theme: {
                      "--toastColor": "mintcream",
                      "--toastBackground": "red",
                      "--toastBarBackground": "#2F855A",
                    },
                  });
            })
          }}
          class="flex space-x-3 w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          >
          {#if loading}

            <p>Launching Rune</p>
            <div role="status">
              <svg aria-hidden="true" class="inline w-5 h-5 text-gray-200 animate-spin dark:text-gray-600 fill-gray-600 dark:fill-gray-300" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                  <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
              </svg>
              <span class="sr-only">Loading...</span>
          </div>

          {:else}
          <p>Launch Rune</p>
          
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-6 h-6"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M15.59 14.37a6 6 0 0 1-5.84 7.38v-4.8m5.84-2.58a14.98 14.98 0 0 0 6.16-12.12A14.98 14.98 0 0 0 9.631 8.41m5.96 5.96a14.926 14.926 0 0 1-5.841 2.58m-.119-8.54a6 6 0 0 0-7.381 5.84h4.8m2.581-5.84a14.927 14.927 0 0 0-2.58 5.84m2.699 2.7c-.103.021-.207.041-.311.06a15.09 15.09 0 0 1-2.448-2.448 14.9 14.9 0 0 1 .06-.312m-2.24 2.39a4.493 4.493 0 0 0-1.757 4.306 4.493 4.493 0 0 0 4.306-1.758M16.5 9a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z"
            />
          </svg>
          {/if}
          
        </button>
      </div>
    </div>
  </div>
</div>
