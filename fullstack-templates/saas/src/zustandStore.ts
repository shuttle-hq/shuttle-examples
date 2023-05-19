import { create } from 'zustand'
import {persist, createJSONStorage} from 'zustand/middleware'

interface AcctState {
	name: string,
	changeName: (to: string) => void,
	email: string,
	changeEmail: (to: string) => void,
}

export const accountStore = create<AcctState>()(persist((set, get) => ({
	name: "",
	changeName: (to) => set({name: to}),
	email: "",
	changeEmail: (to) => set({email: to}),
	}),
	
	{name: 'user',
	storage: createJSONStorage(() => sessionStorage),
}

)
)