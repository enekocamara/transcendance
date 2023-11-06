<script>
	import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Footer, FooterCopyright, FooterLinkGroup, FooterLink } from 'flowbite-svelte';
</script>
<head>
	<script src="https://unpkg.com/@themesberg/flowbite@1.3.0/dist/flowbite.bundle.js"></script>
</head>

<Navbar let:hidden let:toggle>
	<NavBrand href="/">
	  <img
		src="https://flowbite.com/docs/images/logo.svg"
		class="mr-3 h-6 sm:h-9"
		alt="Flowbite Logo"
	  />
	  <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
		Flowbite
	  </span>
	</NavBrand>
	<NavHamburger on:click={toggle} />
	<NavUl {hidden}>
	  <NavLi href="/" active={true}>Home</NavLi>
	  <NavLi href="/about">About</NavLi>
	  <NavLi href="/services">Services</NavLi>
	  <NavLi href="/pricing">Pricing</NavLi>
	  <NavLi href="/contact">Contact</NavLi>
	</NavUl>
</Navbar>

<h1>Este es el index</h1>
<a href="/register">Sign_up</a><br>
<a href="/login">Login</a>

<Footer>
	<FooterCopyright href="/" by="Flowbite™" year={2022} />
	<FooterLinkGroup ulClass="flex flex-wrap items-center mt-3 text-sm text-gray-500 dark:text-gray-400 sm:mt-0">
	  <FooterLink href="/">About</FooterLink>
	  <FooterLink href="/">Privacy Policy</FooterLink>
	  <FooterLink href="/">Licensing</FooterLink>
	  <FooterLink href="/">Contact</FooterLink>
	</FooterLinkGroup>
  </Footer>