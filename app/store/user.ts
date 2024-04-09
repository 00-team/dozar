import { createStore } from 'solid-js/store'

type UserAddress = {
    latitude: number
    longitude: number
    country: string
    state: string
    city: string
    postal: string
    detail: string
}

type UserModel = {
    name: string
    pic: string | null
    phone: string | null
    token: string
    wallet: number
    in_hold: number
    admin: boolean
    address: UserAddress
}

const [user, setUser] = createStore<UserModel>({
    name: 'نام پیش فرض',
    pic: null,
    phone: null,
    wallet: 0,
    in_hold: 0,
    admin: false,
    address: {
        city: '',
        country: '',
        detail: '',
        latitude: 0,
        longitude: 0,
        postal: '',
        state: '',
    },

    // debug
    token: '',
})

export { user, setUser, type UserAddress, type UserModel }
