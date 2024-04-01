/*
This file is auto-generated from the public API of the zstd library.
It is released under the same BSD license.

BSD License

For Zstandard software

Copyright (c) Meta Platforms, Inc. and affiliates. All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
are permitted provided that the following conditions are met:

 * Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

 * Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

 * Neither the name Facebook, nor Meta, nor the names of its contributors may
   be used to endorse or promote products derived from this software without
   specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* automatically generated by rust-bindgen 0.64.0 */

pub const ZDICT_DICTSIZE_MIN: u32 = 256;
pub const ZDICT_CONTENTSIZE_MIN: u32 = 128;
extern "C" {
    #[doc = " ZDICT_trainFromBuffer():\n  Train a dictionary from an array of samples.\n  Redirect towards ZDICT_optimizeTrainFromBuffer_fastCover() single-threaded, with d=8, steps=4,\n  f=20, and accel=1.\n  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,\n  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.\n  The resulting dictionary will be saved into `dictBuffer`.\n @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)\n          or an error code, which can be tested with ZDICT_isError().\n  Note:  Dictionary training will fail if there are not enough samples to construct a\n         dictionary, or if most of the samples are too small (< 8 bytes being the lower limit).\n         If dictionary training fails, you should use zstd without a dictionary, as the dictionary\n         would've been ineffective anyways. If you believe your samples would benefit from a dictionary\n         please open an issue with details, and we can look into it.\n  Note: ZDICT_trainFromBuffer()'s memory usage is about 6 MB.\n  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.\n        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.\n        In general, it's recommended to provide a few thousands samples, though this can vary a lot.\n        It's recommended that total size of all samples be about ~x100 times the target size of dictionary."]
    pub fn ZDICT_trainFromBuffer(
        dictBuffer: *mut libc::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const usize,
        nbSamples: libc::c_uint,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZDICT_params_t {
    #[doc = "< optimize for a specific zstd compression level; 0 means default"]
    pub compressionLevel: libc::c_int,
    #[doc = "< Write log to stderr; 0 = none (default); 1 = errors; 2 = progression; 3 = details; 4 = debug;"]
    pub notificationLevel: libc::c_uint,
    #[doc = "< force dictID value; 0 means auto mode (32-bits random value)\n   NOTE: The zstd format reserves some dictionary IDs for future use.\n         You may use them in private settings, but be warned that they\n         may be used by zstd in a public dictionary registry in the future.\n         These dictionary IDs are:\n           - low range  : <= 32767\n           - high range : >= (2^31)"]
    pub dictID: libc::c_uint,
}
extern "C" {
    #[doc = " ZDICT_finalizeDictionary():\n Given a custom content as a basis for dictionary, and a set of samples,\n finalize dictionary by adding headers and statistics according to the zstd\n dictionary format.\n\n Samples must be stored concatenated in a flat buffer `samplesBuffer`,\n supplied with an array of sizes `samplesSizes`, providing the size of each\n sample in order. The samples are used to construct the statistics, so they\n should be representative of what you will compress with this dictionary.\n\n The compression level can be set in `parameters`. You should pass the\n compression level you expect to use in production. The statistics for each\n compression level differ, so tuning the dictionary for the compression level\n can help quite a bit.\n\n You can set an explicit dictionary ID in `parameters`, or allow us to pick\n a random dictionary ID for you, but we can't guarantee no collisions.\n\n The dstDictBuffer and the dictContent may overlap, and the content will be\n appended to the end of the header. If the header + the content doesn't fit in\n maxDictSize the beginning of the content is truncated to make room, since it\n is presumed that the most profitable content is at the end of the dictionary,\n since that is the cheapest to reference.\n\n `maxDictSize` must be >= max(dictContentSize, ZSTD_DICTSIZE_MIN).\n\n @return: size of dictionary stored into `dstDictBuffer` (<= `maxDictSize`),\n          or an error code, which can be tested by ZDICT_isError().\n Note: ZDICT_finalizeDictionary() will push notifications into stderr if\n       instructed to, using notificationLevel>0.\n NOTE: This function currently may fail in several edge cases including:\n         * Not enough samples\n         * Samples are uncompressible\n         * Samples are all exactly the same"]
    pub fn ZDICT_finalizeDictionary(
        dstDictBuffer: *mut libc::c_void,
        maxDictSize: usize,
        dictContent: *const libc::c_void,
        dictContentSize: usize,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const usize,
        nbSamples: libc::c_uint,
        parameters: ZDICT_params_t,
    ) -> usize;
}
extern "C" {
    pub fn ZDICT_getDictID(
        dictBuffer: *const libc::c_void,
        dictSize: usize,
    ) -> libc::c_uint;
}
extern "C" {
    pub fn ZDICT_getDictHeaderSize(
        dictBuffer: *const libc::c_void,
        dictSize: usize,
    ) -> usize;
}
extern "C" {
    pub fn ZDICT_isError(errorCode: usize) -> libc::c_uint;
}
extern "C" {
    pub fn ZDICT_getErrorName(errorCode: usize) -> *const libc::c_char;
}
#[doc = " ZDICT_cover_params_t:\n  k and d are the only required parameters.\n  For others, value 0 means default."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZDICT_cover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub splitPoint: f64,
    pub shrinkDict: libc::c_uint,
    pub shrinkDictMaxRegression: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZDICT_fastCover_params_t {
    pub k: libc::c_uint,
    pub d: libc::c_uint,
    pub f: libc::c_uint,
    pub steps: libc::c_uint,
    pub nbThreads: libc::c_uint,
    pub splitPoint: f64,
    pub accel: libc::c_uint,
    pub shrinkDict: libc::c_uint,
    pub shrinkDictMaxRegression: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
extern "C" {
    #[doc = " ZDICT_trainFromBuffer_cover():\n  Train a dictionary from an array of samples using the COVER algorithm.\n  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,\n  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.\n  The resulting dictionary will be saved into `dictBuffer`.\n @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)\n          or an error code, which can be tested with ZDICT_isError().\n          See ZDICT_trainFromBuffer() for details on failure modes.\n  Note: ZDICT_trainFromBuffer_cover() requires about 9 bytes of memory for each input byte.\n  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.\n        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.\n        In general, it's recommended to provide a few thousands samples, though this can vary a lot.\n        It's recommended that total size of all samples be about ~x100 times the target size of dictionary."]
    pub fn ZDICT_trainFromBuffer_cover(
        dictBuffer: *mut libc::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const usize,
        nbSamples: libc::c_uint,
        parameters: ZDICT_cover_params_t,
    ) -> usize;
}
extern "C" {
    #[doc = " ZDICT_optimizeTrainFromBuffer_cover():\n The same requirements as above hold for all the parameters except `parameters`.\n This function tries many parameter combinations and picks the best parameters.\n `*parameters` is filled with the best parameters found,\n dictionary constructed with those parameters is stored in `dictBuffer`.\n\n All of the parameters d, k, steps are optional.\n If d is non-zero then we don't check multiple values of d, otherwise we check d = {6, 8}.\n if steps is zero it defaults to its default value.\n If k is non-zero then we don't check multiple values of k, otherwise we check steps values in [50, 2000].\n\n @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)\n          or an error code, which can be tested with ZDICT_isError().\n          On success `*parameters` contains the parameters selected.\n          See ZDICT_trainFromBuffer() for details on failure modes.\n Note: ZDICT_optimizeTrainFromBuffer_cover() requires about 8 bytes of memory for each input byte and additionally another 5 bytes of memory for each byte of memory for each thread."]
    pub fn ZDICT_optimizeTrainFromBuffer_cover(
        dictBuffer: *mut libc::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const usize,
        nbSamples: libc::c_uint,
        parameters: *mut ZDICT_cover_params_t,
    ) -> usize;
}
extern "C" {
    #[doc = " ZDICT_trainFromBuffer_fastCover():\n  Train a dictionary from an array of samples using a modified version of COVER algorithm.\n  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,\n  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.\n  d and k are required.\n  All other parameters are optional, will use default values if not provided\n  The resulting dictionary will be saved into `dictBuffer`.\n @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)\n          or an error code, which can be tested with ZDICT_isError().\n          See ZDICT_trainFromBuffer() for details on failure modes.\n  Note: ZDICT_trainFromBuffer_fastCover() requires 6 * 2^f bytes of memory.\n  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.\n        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.\n        In general, it's recommended to provide a few thousands samples, though this can vary a lot.\n        It's recommended that total size of all samples be about ~x100 times the target size of dictionary."]
    pub fn ZDICT_trainFromBuffer_fastCover(
        dictBuffer: *mut libc::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const usize,
        nbSamples: libc::c_uint,
        parameters: ZDICT_fastCover_params_t,
    ) -> usize;
}
extern "C" {
    #[doc = " ZDICT_optimizeTrainFromBuffer_fastCover():\n The same requirements as above hold for all the parameters except `parameters`.\n This function tries many parameter combinations (specifically, k and d combinations)\n and picks the best parameters. `*parameters` is filled with the best parameters found,\n dictionary constructed with those parameters is stored in `dictBuffer`.\n All of the parameters d, k, steps, f, and accel are optional.\n If d is non-zero then we don't check multiple values of d, otherwise we check d = {6, 8}.\n if steps is zero it defaults to its default value.\n If k is non-zero then we don't check multiple values of k, otherwise we check steps values in [50, 2000].\n If f is zero, default value of 20 is used.\n If accel is zero, default value of 1 is used.\n\n @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)\n          or an error code, which can be tested with ZDICT_isError().\n          On success `*parameters` contains the parameters selected.\n          See ZDICT_trainFromBuffer() for details on failure modes.\n Note: ZDICT_optimizeTrainFromBuffer_fastCover() requires about 6 * 2^f bytes of memory for each thread."]
    pub fn ZDICT_optimizeTrainFromBuffer_fastCover(
        dictBuffer: *mut libc::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const usize,
        nbSamples: libc::c_uint,
        parameters: *mut ZDICT_fastCover_params_t,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ZDICT_legacy_params_t {
    pub selectivityLevel: libc::c_uint,
    pub zParams: ZDICT_params_t,
}
extern "C" {
    #[doc = " ZDICT_trainFromBuffer_legacy():\n  Train a dictionary from an array of samples.\n  Samples must be stored concatenated in a single flat buffer `samplesBuffer`,\n  supplied with an array of sizes `samplesSizes`, providing the size of each sample, in order.\n  The resulting dictionary will be saved into `dictBuffer`.\n `parameters` is optional and can be provided with values set to 0 to mean \"default\".\n @return: size of dictionary stored into `dictBuffer` (<= `dictBufferCapacity`)\n          or an error code, which can be tested with ZDICT_isError().\n          See ZDICT_trainFromBuffer() for details on failure modes.\n  Tips: In general, a reasonable dictionary has a size of ~ 100 KB.\n        It's possible to select smaller or larger size, just by specifying `dictBufferCapacity`.\n        In general, it's recommended to provide a few thousands samples, though this can vary a lot.\n        It's recommended that total size of all samples be about ~x100 times the target size of dictionary.\n  Note: ZDICT_trainFromBuffer_legacy() will send notifications into stderr if instructed to, using notificationLevel>0."]
    pub fn ZDICT_trainFromBuffer_legacy(
        dictBuffer: *mut libc::c_void,
        dictBufferCapacity: usize,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const usize,
        nbSamples: libc::c_uint,
        parameters: ZDICT_legacy_params_t,
    ) -> usize;
}
extern "C" {
    pub fn ZDICT_addEntropyTablesFromBuffer(
        dictBuffer: *mut libc::c_void,
        dictContentSize: usize,
        dictBufferCapacity: usize,
        samplesBuffer: *const libc::c_void,
        samplesSizes: *const usize,
        nbSamples: libc::c_uint,
    ) -> usize;
}
