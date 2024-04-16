<script>
  import "../../index.css";
  import { btc_address, is_authenticated } from "$lib/store";
  import { onMount } from "svelte";
  import { SvelteToast } from "@zerodevx/svelte-toast";
  import {
    authenticate,
    get_btc_deposit_address,
    logout,
    user_authenticated,
  } from "$lib/sdk";

  onMount(() => {
    is_authenticated.subscribe((data) => {
      if (data) {
        get_btc_deposit_address().then((addr) => {
          btc_address.set(addr);
        });
      }
    });
    let task = async () => {
      if (!(await user_authenticated())) {
        return;
      }

      is_authenticated.set(true);
      let addr = await get_btc_deposit_address();
      btc_address.set(addr);
    };

    task();
  });
</script>

<SvelteToast />
<div>
  <!-- Off-canvas menu for mobile, show/hide based on off-canvas menu state. -->
  <div class="relative z-50 lg:hidden" role="dialog" aria-modal="true">
    <!--
          Off-canvas menu backdrop, show/hide based on off-canvas menu state.
    
          Entering: "transition-opacity ease-linear duration-300"
            From: "opacity-0"
            To: "opacity-100"
          Leaving: "transition-opacity ease-linear duration-300"
            From: "opacity-100"
            To: "opacity-0"
        -->
    <div class="fixed inset-0 bg-gray-900/80"></div>

    <div class="fixed inset-0 flex">
      <!--
            Off-canvas menu, show/hide based on off-canvas menu state.
    
            Entering: "transition ease-in-out duration-300 transform"
              From: "-translate-x-full"
              To: "translate-x-0"
            Leaving: "transition ease-in-out duration-300 transform"
              From: "translate-x-0"
              To: "-translate-x-full"
          -->
      <div class="relative mr-16 flex w-full max-w-xs flex-1">
        <!--
              Close button, show/hide based on off-canvas menu state.
    
              Entering: "ease-in-out duration-300"
                From: "opacity-0"
                To: "opacity-100"
              Leaving: "ease-in-out duration-300"
                From: "opacity-100"
                To: "opacity-0"
            -->
        <div class="absolute left-full top-0 flex w-16 justify-center pt-5">
          <button type="button" class="-m-2.5 p-2.5">
            <span class="sr-only">Close sidebar</span>
            <svg
              class="h-6 w-6 text-white"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              aria-hidden="true"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        <!-- Sidebar component, swap this element with another sidebar if you like -->
        <div
          class="flex grow flex-col gap-y-5 overflow-y-auto bg-gray-900 px-6 pb-2 ring-1 ring-white/10"
        >
          <div class="flex h-16 shrink-0 items-center">
            <img
              class="h-8 w-auto"
              src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500"
              alt="Your Company"
            />
          </div>
          <nav class="flex flex-1 flex-col">
            <ul role="list" class="flex flex-1 flex-col gap-y-7">
              <li>
                <ul role="list" class="-mx-2 space-y-1">
                  <li>
                    <!-- Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" -->
                    <a
                      href="#"
                      class="bg-gray-800 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                    >
                      <svg
                        class="h-6 w-6 shrink-0"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        aria-hidden="true"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
                        />
                      </svg>
                      Dashboard
                    </a>
                  </li>
                  <li>
                    <a
                      href="#"
                      class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                    >
                      <svg
                        class="h-6 w-6 shrink-0"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        aria-hidden="true"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"
                        />
                      </svg>
                      Team
                    </a>
                  </li>
                  <li>
                    <a
                      href="#"
                      class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                    >
                      <svg
                        class="h-6 w-6 shrink-0"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        aria-hidden="true"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
                        />
                      </svg>
                      Projects
                    </a>
                  </li>
                  <li>
                    <a
                      href="#"
                      class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                    >
                      <svg
                        class="h-6 w-6 shrink-0"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        aria-hidden="true"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5"
                        />
                      </svg>
                      Calendar
                    </a>
                  </li>
                  <li>
                    <a
                      href="#"
                      class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                    >
                      <svg
                        class="h-6 w-6 shrink-0"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        aria-hidden="true"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 01-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 011.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 00-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 01-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 00-3.375-3.375h-1.5a1.125 1.125 0 01-1.125-1.125v-1.5a3.375 3.375 0 00-3.375-3.375H9.75"
                        />
                      </svg>
                      Documents
                    </a>
                  </li>
                  <li>
                    <a
                      href="#"
                      class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                    >
                      <svg
                        class="h-6 w-6 shrink-0"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        aria-hidden="true"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d="M10.5 6a7.5 7.5 0 107.5 7.5h-7.5V6z"
                        />
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d="M13.5 10.5H21A7.5 7.5 0 0013.5 3v7.5z"
                        />
                      </svg>
                      Reports
                    </a>
                  </li>
                </ul>
              </li>
              <li>
                <div class="text-xs font-semibold leading-6 text-gray-400">
                  Your teams
                </div>
                <ul role="list" class="-mx-2 mt-2 space-y-1">
                  <li>
                    <!-- Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" -->
                    <a
                      href="#"
                      class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                    >
                      <span
                        class="flex h-6 w-6 shrink-0 items-center justify-center rounded-lg border border-gray-700 bg-gray-800 text-[0.625rem] font-medium text-gray-400 group-hover:text-white"
                        >H</span
                      >
                      <span class="truncate">Heroicons</span>
                    </a>
                  </li>
                  <li>
                    <a
                      href="#"
                      class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                    >
                      <span
                        class="flex h-6 w-6 shrink-0 items-center justify-center rounded-lg border border-gray-700 bg-gray-800 text-[0.625rem] font-medium text-gray-400 group-hover:text-white"
                        >T</span
                      >
                      <span class="truncate">Tailwind Labs</span>
                    </a>
                  </li>
                  <li>
                    <a
                      href="#"
                      class="text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                    >
                      <span
                        class="flex h-6 w-6 shrink-0 items-center justify-center rounded-lg border border-gray-700 bg-gray-800 text-[0.625rem] font-medium text-gray-400 group-hover:text-white"
                        >W</span
                      >
                      <span class="truncate">Workcation</span>
                    </a>
                  </li>
                </ul>
              </li>
            </ul>
          </nav>
        </div>
      </div>
    </div>
  </div>

  <!-- Static sidebar for desktop -->
  <div class="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col">
    <!-- Sidebar component, swap this element with another sidebar if you like -->
    <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-gray-900 px-6">
      <div class="flex h-16 shrink-0 items-center">
        <img
          class="h-8 w-auto"
          src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500"
          alt="Your Company"
        />
      </div>
      <nav class="flex flex-1 flex-col">
        <ul role="list" class="flex flex-1 flex-col gap-y-7">
          <li>
            <ul role="list" class="-mx-2 space-y-1">
              <li>
                <!-- Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" -->
                <a
                  href="/launcher/"
                  class="hover:bg-gray-800 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                >
                  <svg
                    class="h-6 w-6 shrink-0"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    aria-hidden="true"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
                    />
                  </svg>
                  Create Rune
                </a>
              </li>

              <li>
                <!-- Current: "bg-gray-800 text-white", Default: "text-gray-400 hover:text-white hover:bg-gray-800" -->
                <a
                  href="/launcher/withdraw"
                  class="hover:bg-gray-800 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
                >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 8.25h19.5M2.25 9h19.5m-16.5 5.25h6m-6 2.25h3m-3.75 3h15a2.25 2.25 0 0 0 2.25-2.25V6.75A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25v10.5A2.25 2.25 0 0 0 4.5 19.5Z" />
                </svg>
                
                  Withdraw
                </a>
              </li>
            </ul>
          </li>
          <li></li>
        </ul>
      </nav>
    </div>
  </div>

  <div
    class="sticky top-0 z-40 flex items-center gap-x-6 bg-gray-900 px-4 py-4 shadow-sm sm:px-6 lg:hidden"
  >
    <button type="button" class="-m-2.5 p-2.5 text-gray-400 lg:hidden">
      <span class="sr-only">Open sidebar</span>
      <svg
        class="h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        aria-hidden="true"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
        />
      </svg>
    </button>
    <div class="flex-1 text-sm font-semibold leading-6 text-white">
      Dashboard
    </div>
    <a href="#">
      <span class="sr-only">Your profile</span>
      <img
        class="h-8 w-8 rounded-full bg-gray-800"
        src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
        alt=""
      />
    </a>
  </div>

  <main class="lg:pl-72 bg-gray-200">
    <header>
      <nav
        class="bg-white border-gray-200 px-4 lg:px-6 py-2.5 dark:bg-gray-800"
      >
        <div class="grid grid-cols-3 items-center mx-auto max-w-screen-xl">
          <a
            href="/"
            class="flex items-center lg:justify-center lg:order-2"
          >
            <img
              src="https://flowbite.com/docs/images/logo.svg"
              class="mr-3 h-6 sm:h-9"
              alt="Flowbite Logo"
            />
            <span
              class="self-center text-xl font-semibold whitespace-nowrap dark:text-white"
              >Runity</span
            >
          </a>
          <div
            class="flex col-span-2 justify-end items-center lg:order-3 lg:col-span-1"
          >
            {#if $is_authenticated}
              <button
                on:click={(e) => {
                  logout().then(() => {
                    is_authenticated.set(false);
                  });
                }}
                class="text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-4 py-2 lg:py-2.5 mr-2 dark:bg-red-600 dark:hover:bg-red-700 focus:outline-none dark:focus:ring-red-800"
                >Disconnect</button
              >
            {:else}
              <button
                on:click={(e) => {
                  authenticate(() => {
                    is_authenticated.set(true);
                  }).then(() => {
                    console.log("Authenticated");
                  });
                }}
                class="text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm px-4 py-2 lg:py-2.5 mr-2 dark:bg-primary-600 dark:hover:bg-primary-700 focus:outline-none dark:focus:ring-primary-800"
                >Sign in with Internet Identity</button
              >
            {/if}

            <button
              data-collapse-toggle="mobile-menu-2"
              type="button"
              class="inline-flex items-center p-2 ml-1 text-sm text-gray-500 rounded-lg lg:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
              aria-controls="mobile-menu-2"
              aria-expanded="false"
            >
              <span class="sr-only">Open main menu</span>
              <svg
                class="w-6 h-6"
                fill="currentColor"
                viewBox="0 0 20 20"
                xmlns="http://www.w3.org/2000/svg"
                ><path
                  fill-rule="evenodd"
                  d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
                  clip-rule="evenodd"
                ></path></svg
              >
              <svg
                class="hidden w-6 h-6"
                fill="currentColor"
                viewBox="0 0 20 20"
                xmlns="http://www.w3.org/2000/svg"
                ><path
                  fill-rule="evenodd"
                  d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                  clip-rule="evenodd"
                ></path></svg
              >
            </button>
          </div>
          <div
            class="hidden col-span-3 justify-between items-center w-full lg:flex lg:w-auto lg:order-1 lg:col-span-1"
            id="mobile-menu-2"
          ></div>
        </div>
      </nav>
    </header>
    <section class="py-10">
      <slot />
    </section>

    <footer class="p-4 bg-white md:p-8 lg:p-10 dark:bg-gray-800">
      <div class="mx-auto max-w-screen-xl text-center">
        <a
          href="#"
          class="flex justify-center items-center text-2xl font-semibold text-gray-900 dark:text-white"
        >
          <svg
            class="mr-2 h-8"
            viewBox="0 0 33 33"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M25.2696 13.126C25.1955 13.6364 24.8589 14.3299 24.4728 14.9328C23.9856 15.6936 23.2125 16.2264 22.3276 16.4114L18.43 17.2265C17.8035 17.3575 17.2355 17.6853 16.8089 18.1621L14.2533 21.0188C13.773 21.5556 13.4373 21.4276 13.4373 20.7075C13.4315 20.7342 12.1689 23.9903 15.5149 25.9202C16.8005 26.6618 18.6511 26.3953 19.9367 25.6538L26.7486 21.7247C29.2961 20.2553 31.0948 17.7695 31.6926 14.892C31.7163 14.7781 31.7345 14.6639 31.7542 14.5498L25.2696 13.126Z"
              fill="url(#paint0_linear_11430_22515)"
            /><path
              d="M23.5028 9.20133C24.7884 9.94288 25.3137 11.0469 25.3137 12.53C25.3137 12.7313 25.2979 12.9302 25.2694 13.1261L28.0141 14.3051L31.754 14.5499C32.2329 11.7784 31.2944 8.92561 29.612 6.65804C28.3459 4.9516 26.7167 3.47073 24.7581 2.34097C23.167 1.42325 21.5136 0.818599 19.8525 0.486816L17.9861 2.90382L17.3965 5.67918L23.5028 9.20133Z"
              fill="url(#paint1_linear_11430_22515)"
            /><path
              d="M1.5336 11.2352C1.5329 11.2373 1.53483 11.238 1.53556 11.2358C1.67958 10.8038 1.86018 10.3219 2.08564 9.80704C3.26334 7.11765 5.53286 5.32397 8.32492 4.40943C11.117 3.49491 14.1655 3.81547 16.7101 5.28323L17.3965 5.67913L19.8525 0.486761C12.041 -1.07341 4.05728 3.51588 1.54353 11.2051C1.54233 11.2087 1.53796 11.2216 1.5336 11.2352Z"
              fill="url(#paint2_linear_11430_22515)"
            /><path
              d="M19.6699 25.6538C18.3843 26.3953 16.8003 26.3953 15.5147 25.6538C15.3402 25.5531 15.1757 25.4399 15.0201 25.3174L12.7591 26.8719L10.8103 30.0209C12.9733 31.821 15.7821 32.3997 18.589 32.0779C20.7013 31.8357 22.7995 31.1665 24.7582 30.0368C26.3492 29.1191 27.7 27.9909 28.8182 26.7195L27.6563 23.8962L25.7762 22.1316L19.6699 25.6538Z"
              fill="url(#paint3_linear_11430_22515)"
            /><path
              d="M15.0201 25.3175C14.0296 24.5373 13.4371 23.3406 13.4371 22.0588V21.931V11.2558C13.4371 10.6521 13.615 10.5494 14.1384 10.8513C13.3323 10.3864 11.4703 8.79036 9.17118 10.1165C7.88557 10.858 6.8269 12.4949 6.8269 13.978V21.8362C6.8269 24.775 8.34906 27.8406 10.5445 29.7966C10.6313 29.874 10.7212 29.9469 10.8103 30.0211L15.0201 25.3175Z"
              fill="url(#paint4_linear_11430_22515)"
            /><path
              d="M28.6604 5.49565C28.6589 5.49395 28.6573 5.49532 28.6589 5.49703C28.9613 5.83763 29.2888 6.23485 29.6223 6.68734C31.3648 9.05099 32.0158 12.0447 31.4126 14.9176C30.8093 17.7906 29.0071 20.2679 26.4625 21.7357L25.7761 22.1316L28.8181 26.7195C34.0764 20.741 34.09 11.5388 28.6815 5.51929C28.6789 5.51641 28.67 5.50622 28.6604 5.49565Z"
              fill="url(#paint5_linear_11430_22515)"
            /><path
              d="M7.09355 13.978C7.09354 12.4949 7.88551 11.1244 9.17113 10.3829C9.34564 10.2822 9.52601 10.1965 9.71002 10.1231L9.49304 7.38962L7.96861 4.26221C5.32671 5.23364 3.1897 7.24125 2.06528 9.83067C1.2191 11.7793 0.75001 13.9294 0.75 16.1888C0.75 18.0243 1.05255 19.7571 1.59553 21.3603L4.62391 21.7666L7.09355 21.0223V13.978Z"
              fill="url(#paint6_linear_11430_22515)"
            /><path
              d="M9.71016 10.1231C10.8817 9.65623 12.2153 9.74199 13.3264 10.3829L13.4372 10.4468L22.3326 15.5777C22.9566 15.9376 22.8999 16.2918 22.1946 16.4392L22.7078 16.332C23.383 16.1908 23.9999 15.8457 24.4717 15.3428C25.2828 14.4782 25.5806 13.4351 25.5806 12.5299C25.5806 11.0468 24.7886 9.67634 23.503 8.93479L16.6911 5.00568C14.1436 3.53627 11.0895 3.22294 8.29622 4.14442C8.18572 4.18087 8.07756 4.2222 7.96875 4.26221L9.71016 10.1231Z"
              fill="url(#paint7_linear_11430_22515)"
            /><path
              d="M20.0721 31.8357C20.0744 31.8352 20.0739 31.8332 20.0717 31.8337C19.6252 31.925 19.1172 32.0097 18.5581 32.0721C15.638 32.3978 12.7174 31.4643 10.5286 29.5059C8.33986 27.5474 7.09347 24.7495 7.09348 21.814L7.09347 21.0222L1.59546 21.3602C4.1488 28.8989 12.1189 33.5118 20.0411 31.8421C20.0449 31.8413 20.0582 31.8387 20.0721 31.8357Z"
              fill="url(#paint8_linear_11430_22515)"
            />
            <defs>
              <linearGradient
                id="paint0_linear_11430_22515"
                x1="20.8102"
                y1="23.9532"
                x2="23.9577"
                y2="12.9901"
                gradientUnits="userSpaceOnUse"
                ><stop stop-color="#1724C9" /><stop
                  offset="1"
                  stop-color="#1C64F2"
                /></linearGradient
              >
              <linearGradient
                id="paint1_linear_11430_22515"
                x1="28.0593"
                y1="10.5837"
                x2="19.7797"
                y2="2.33321"
                gradientUnits="userSpaceOnUse"
                ><stop stop-color="#1C64F2" /><stop
                  offset="1"
                  stop-color="#0092FF"
                /></linearGradient
              >
              <linearGradient
                id="paint2_linear_11430_22515"
                x1="16.9145"
                y1="5.2045"
                x2="4.42432"
                y2="5.99375"
                gradientUnits="userSpaceOnUse"
                ><stop stop-color="#0092FF" /><stop
                  offset="1"
                  stop-color="#45B2FF"
                /></linearGradient
              >
              <linearGradient
                id="paint3_linear_11430_22515"
                x1="16.0698"
                y1="28.846"
                x2="27.2866"
                y2="25.8192"
                gradientUnits="userSpaceOnUse"
                ><stop stop-color="#1C64F2" /><stop
                  offset="1"
                  stop-color="#0092FF"
                /></linearGradient
              >
              <linearGradient
                id="paint4_linear_11430_22515"
                x1="8.01881"
                y1="15.8661"
                x2="15.9825"
                y2="24.1181"
                gradientUnits="userSpaceOnUse"
                ><stop stop-color="#1724C9" /><stop
                  offset="1"
                  stop-color="#1C64F2"
                /></linearGradient
              >
              <linearGradient
                id="paint5_linear_11430_22515"
                x1="26.2004"
                y1="21.8189"
                x2="31.7569"
                y2="10.6178"
                gradientUnits="userSpaceOnUse"
                ><stop stop-color="#0092FF" /><stop
                  offset="1"
                  stop-color="#45B2FF"
                /></linearGradient
              >
              <linearGradient
                id="paint6_linear_11430_22515"
                x1="6.11387"
                y1="9.31427"
                x2="3.14054"
                y2="20.4898"
                gradientUnits="userSpaceOnUse"
                ><stop stop-color="#1C64F2" /><stop
                  offset="1"
                  stop-color="#0092FF"
                /></linearGradient
              >
              <linearGradient
                id="paint7_linear_11430_22515"
                x1="21.2932"
                y1="8.78271"
                x2="10.4278"
                y2="11.488"
                gradientUnits="userSpaceOnUse"
                ><stop stop-color="#1724C9" /><stop
                  offset="1"
                  stop-color="#1C64F2"
                /></linearGradient
              >
              <linearGradient
                id="paint8_linear_11430_22515"
                x1="7.15667"
                y1="21.5399"
                x2="14.0824"
                y2="31.9579"
                gradientUnits="userSpaceOnUse"
                ><stop stop-color="#0092FF" /><stop
                  offset="1"
                  stop-color="#45B2FF"
                /></linearGradient
              >
            </defs>
          </svg>
          Runity
        </a>
        <!-- <p class="my-6 text-gray-500 dark:text-gray-400">Open-source library of over 400+ web components and interactive elements built for better web.</p> -->
        <!-- <ul class="flex flex-wrap justify-center items-center mb-6 text-gray-900 dark:text-white">
                <li>
                    <a href="#" class="mr-4 hover:underline md:mr-6 ">About</a>
                </li>
                <li>
                    <a href="#" class="mr-4 hover:underline md:mr-6">Premium</a>
                </li>
                <li>
                    <a href="#" class="mr-4 hover:underline md:mr-6 ">Campaigns</a>
                </li>
                <li>
                    <a href="#" class="mr-4 hover:underline md:mr-6">Blog</a>
                </li>
                <li>
                    <a href="#" class="mr-4 hover:underline md:mr-6">Affiliate Program</a>
                </li>
                <li>
                    <a href="#" class="mr-4 hover:underline md:mr-6">FAQs</a>
                </li>
                <li>
                    <a href="#" class="mr-4 hover:underline md:mr-6">Contact</a>
                </li>
            </ul> -->
        <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400"
          >© 2021-2022 <a href="#" class="hover:underline">Flowbite™</a>. All
          Rights Reserved.</span
        >
      </div>
    </footer>
  </main>
</div>
