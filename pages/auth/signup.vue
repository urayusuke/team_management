<template>
  <amplify-authenticator>
    <amplify-sign-up slot="sign-up" />
  </amplify-authenticator>
</template>

<script>
  export default {
    name: 'signup',
    data(){
      return {
        formFields: [
          {
            type: 'email',
            label: 'Custom Email Label',
            placeholder: 'Custom email placeholder',
            inputProps: { required: true, autocomplete: 'username' },
          },
          {
            type: 'password',
            label: 'Custom Password Label',
            placeholder: 'Custom password placeholder',
            inputProps: { required: true, autocomplete: 'new-password' },
          },
        ]
      }
    }
  }
</script>