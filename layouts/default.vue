<script lang="ts" src="./default.ts"></script>

<template>
  <v-app>
    <header>
      <v-app-bar>
        <v-app-bar-nav-icon @click="sideBarDrawer = !sideBarDrawer"></v-app-bar-nav-icon>
        <v-toolbar-title>{{ title }}</v-toolbar-title>
      </v-app-bar>
    </header>
    <v-navigation-drawer v-model="sideBarDrawer" fixed temporary>
      <v-sheet
        color="grey lighten-4"
        class="pa-4"
      >
        <v-avatar
          class="mb-4"
          color="grey darken-1"
          size="64"
        ></v-avatar>
        <div>
          <v-btn dark color="primary">ログイン</v-btn>
        </div>
      </v-sheet>


      <v-list-item-group>
        <v-list-item v-for="l in sideBarList" :key="l.id">
          <v-icon>{{ l.icon }}</v-icon>
          <v-list-item-content>
            <v-list-item-title>{{ l.title }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list-item-group>

    </v-navigation-drawer>

    <v-main>
      <v-container>
        <Nuxt />
      </v-container>
    </v-main>

    <v-footer></v-footer>
  </v-app>
</template>
