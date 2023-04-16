import Vue from 'vue'
import { Amplify, API } from 'aws-amplify'
import awsExports from '../src/aws-exports'
import { applyPolyfills, defineCustomElements } from '@aws-amplify/ui-components/loader'

Amplify.configure(awsExports);
Vue.use(Amplify)

applyPolyfills().then(() => {
  defineCustomElements(window);
});