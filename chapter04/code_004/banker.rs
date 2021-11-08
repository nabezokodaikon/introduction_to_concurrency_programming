struct Resource<const NRES: usize, const NTH: usize> {
    available: [usize; NRES],
    allocation: [[usize; NRES]; NTH],
    mac: [[usize; NRES]; NTH],
}
