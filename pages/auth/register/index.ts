import { Component, Vue } from 'vue-property-decorator'
import RegisterForm from "~/components/module/auth/RegisterForm";

@Component({
  components:{
    RegisterForm,
  },
})

export default class RegisterFormComponent extends Vue {

}
