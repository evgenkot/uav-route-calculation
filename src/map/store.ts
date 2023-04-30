import { writable } from 'svelte/store';
import type { Camera, Uav } from './types';

export const selectedUav = writable<Uav | null>(null);
export const selectedCamera = writable<Camera | null>(null);
export const altitudeValue = writable<number>(0);
