import { Component } from 'solid-js'
import { createStore } from 'solid-js/store'

import './style/myprofile.scss'

import defaultImg from '!/static/imgs/default_profile.webp'

const IMAGE_MIMETYPE = [
    'image/png',
    'image/jpeg',
    'image/jpg',
    'image/gif',
    'image/webm',
]

export const MyProfile: Component<{}> = props => {
    const [profile, setProfile] = createStore({
        username: '',
        img: '',
    })

    return (
        <div class='profile-container'>
            <h2 class='title'>پروفایل من</h2>
            <label for='profile-inp' class='profile-img'>
                <img
                    src={profile.img || defaultImg}
                    decoding={'async'}
                    loading='lazy'
                    alt=''
                    draggable={false}
                />
                <input
                    type='file'
                    id='profile-inp'
                    accept='.jpg, .jpeg, .png, image/jpg, image/jpeg, image/png'
                    onchange={e => {
                        if (!e.target.files || !e.target.files[0]) return

                        const file = e.target.files[0]

                        if (!IMAGE_MIMETYPE.includes(file.type)) return

                        const url = URL.createObjectURL(file)

                        setProfile({
                            img: url,
                        })
                    }}
                />
            </label>
            <div class='profile-name'></div>
            <button class='cta title_smaller'>تایید</button>
        </div>
    )
}
