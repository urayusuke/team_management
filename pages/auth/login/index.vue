<template>
  <div>
    <v-card class="d-flex flex-column my-6 mx-auto" width="374" color="#fff">
      <v-form ref="form" >
        <v-card-title class="d-flex justify-center pa-0 mt-6 mb-3">ログイン</v-card-title>
        <v-card-text class="d-flex justify-center flex-column">
          <div class="mx-9">
            <v-text-field
              label="メールアドレス"
              placeholder="mail@example.com"
              outlined
              dense
            ></v-text-field>
            <v-text-field
              label="パスワード"
              placeholder="8文字以上の半角英数記号"
              outlined
              dense
            ></v-text-field>
          </div>
          <div class="text-center">
            <v-btn class="primary" >ログイン</v-btn>
          </div>
          <v-btn
            class="fill-width mt-6 text-capitalize caption mx-4 mb-6"
            rounded
            height="48px"
            outlined
            style="border-color: #979797"
          >
            <img
              class="button-logo-img mr-4"
              src="https://madeby.google.com/static/images/google_g_logo.svg"
              style="height: 24px"
            />
            Googleでログイン
          </v-btn>
          <p class="pointer">
            新規登録は
            <nuxt-link to="/auth/register">
              こちら
            </nuxt-link>
          </p>
        </v-card-text>
      </v-form>
    </v-card>
    </div>
  </template>
