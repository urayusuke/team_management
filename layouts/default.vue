<script lang="ts" src="./default.ts"></script>

<template>
  <v-app>
    <v-app-bar>
      <v-app-bar-title>{{ title }}</v-app-bar-title>
    </v-app-bar>
    <v-navigation-drawer>
      <v-sheet
        color="grey lighten-4"
        class="pa-4"
      >
        <v-avatar
          class="mb-4"
          color="grey darken-1"
          size="64"
        ></v-avatar>
      </v-sheet>

      <v-list>
        <v-list-item v-for="(l, i) in sideBarList">
          <v-icon>{{ l.icon }}</v-icon>
          <v-list-item-content>
            <v-list-item-title>{{ l.title }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
    <v-main>
      <v-container>
        <Nuxt />
      </v-container>
    </v-main>
    <v-footer></v-footer>
  </v-app>
</template>