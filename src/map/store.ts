import { writable } from 'svelte/store';
import type { Camera } from './types';

export const selectedUav = writable(null);
export const selectedCamera = writable<Camera | null>(null);
