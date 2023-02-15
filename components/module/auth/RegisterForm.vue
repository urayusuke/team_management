<template>
  <v-form
    ref="form"
    lazy-validation
  >
    <v-text-field
      :counter="10"
      label="Name"
      required
    ></v-text-field>

    <v-text-field
      label="E-mail"
      required
    ></v-text-field>

    <v-select
      label="Item"
      required
    ></v-select>

    <v-checkbox
      label="Do you agree?"
      required
    ></v-checkbox>

    <v-btn
      color="success"
      class="mr-4"
    >
      Validate
    </v-btn>

    <v-btn
      color="error"
      class="mr-4"
    >
      Reset Form
    </v-btn>

    <v-btn
      color="warning"
    >
      Reset Validation
    </v-btn>
  </v-form>
</template>