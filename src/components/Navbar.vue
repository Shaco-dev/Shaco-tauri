<template>
    <nav class="navbar dark:bg-[#2f2f2f] border-b border-gray-300 dark:border-white/10">
        <ul class="nav-links">
            <li>
                <router-link to="/" class="select-none logo-link bg-white dark:bg-[#0f0f0f98] text-gray-700 dark:text-white">
                    <span class="flex items-center gap-2">
                        <img :src="HomeLogo" class="logo-img" />
                        <span class="logo-text">Home</span>
                    </span>
                </router-link>
            </li>
            <li>
                <router-link to="/Languages" class="select-none logo-link bg-white dark:bg-[#0f0f0f98] text-gray-700 dark:text-white">
                    <span class="flex items-center gap-2">
                        <img :src="LanguesLogo" class="logo-img" />
                        <span class="logo-text">Langues</span>
                    </span>
                </router-link>
            </li>
            <li>
                <router-link to="/Assets" class="select-none logo-link bg-white dark:bg-[#0f0f0f98] text-gray-700 dark:text-white">
                    <span class="flex items-center gap-2">
                        <img :src="AssetsLogo" class="logo-img" />
                        <span class="logo-text">Assets</span>
                    </span>
                </router-link>
            </li>
        </ul>
    </nav>
</template>

<script setup lang="ts">
import HomeLogo from '../assets/home.png';
import LanguesLogo from '../assets/langues.png';
import AssetsLogo from '../assets/assets.png';
</script>

<style scoped>
.navbar {
    display: flex;
    justify-content: center;
    padding: 0.5rem 1rem;
    position: sticky;
    top: 0;
    z-index: 1000;
    height: 3.5rem;
}

.nav-links {
    list-style: none;
    display: flex;
    align-items: center;
    gap: 1.5rem;
    margin: 0;
    padding: 0;
}

.nav-links a {
    font-weight: 500;
    text-decoration: none;
    padding: 0.5rem 1.25rem;
    border-radius: 8px;
    border: 1px solid transparent;
    transition: all 0.25s;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
    font-size: 0.9rem;
    display: flex;
    align-items: center;
}

.logo-link {
    padding: 0.25rem 1rem !important;
}

.logo-img {
    height: 2rem;
    width: auto;
    transition: transform 0.2s;
}

.logo-text {
    font-weight: 600;
    letter-spacing: 0.5px;
}

.nav-links a:hover {
    color: #ffbd2e;
    border-color: #ffbd2e;
    transform: translateY(-1px);
}

.nav-links a:hover .logo-img {
    transform: scale(1.05);
}

.nav-links a.router-link-exact-active {
    color: #ffbd2e;
    border-color: #ffbd2e;
}
</style>