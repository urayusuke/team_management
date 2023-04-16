<template>
  <div>
    <div v-if="authState !== 'signedin'">
      <nuxt-link to="/auth/signin">Sign In</nuxt-link>
      <nuxt-link to="/auth/signup">Sign Up</nuxt-link>
    </div>
    <!-- <amplify-authenticator>
      <div v-if="authState === 'signedin' && user">
        <div>Hello, {{user.username}}</div>
      </div>
      <amplify-sign-out></amplify-sign-out>
    </amplify-authenticator> -->
  </div>
</template>

<script>
import { onAuthUIStateChange } from '@aws-amplify/ui-components';
import { AmplifyAuthenticator } from '@aws-amplify/ui-components';

export default {
  name: 'AuthStateApp',
  components: {
    AmplifyAuthenticator
  },
  created() {
    this.unsubscribeAuth = onAuthUIStateChange((authState, authData) => {
      this.authState = authState;
      this.user = authData;
    });
  },
  data() {
    return {
      user: undefined,
      authState: undefined,
      unsubscribeAuth: undefined,
    };
  },
  beforeDestroy() {
    this.unsubscribeAuth();
  },
};


</script>