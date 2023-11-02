import authGuard from './components/authGuard.js'

const routes = {
    '/game':{
        beforeEnter: authGuard,
    }
}