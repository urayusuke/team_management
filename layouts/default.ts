import Vue from 'vue'

export default Vue.extend({
  data () {
    return {
      title: 'TEAM MANAGEMENT',
      sideBarList: [ 
        {
          title: 'お知らせ',
          icon: 'mdi-bullhorn' 
        },
        {
          title: 'メンバー一覧',
          icon: 'mdi-account-group' 
        },
        {
          title: '出 欠',
          icon: 'mdi-calendar-month' 
        },
        {
          title: 'テーマ設定',
          icon: 'mdi-wrench-cog-outline' 
        },
      ] 
    }
  },
})