/* tslint:disable */
/* eslint-disable */
/**
* @param {string} a
* @param {string} b
* @returns {number}
*/
export function powierza(a: string, b: string): number;
/**
* @param {number} font_idx
* @param {string} font_bytes_hex
*/
export function register_font(font_idx: number, font_bytes_hex: string): void;
/**
* @param {number} font_idx
* @returns {boolean}
*/
export function font_register_is_free(font_idx: number): boolean;
/**
* @param {string} text
* @returns {string}
*/
export function find_line_break_positions(text: string): string;
/**
* @param {string} text
* @returns {string}
*/
export function decode_ncrs(text: string): string;
