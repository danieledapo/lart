#include "rust/cxx.h"

#include <memory>

struct V;
struct Path;
struct Geometry;

class Clipper
{
public:
    Clipper();

    void add_subject(Path const & /*subject*/);
    void add_clip(Path const & /*polygon*/);

    Geometry union_();
    Geometry intersection();
    Geometry difference();
    Geometry symmetric_difference();

private:
    struct pimpl;
    std::shared_ptr<pimpl> impl;
};

std::unique_ptr<Clipper> new_clipper();

Geometry buffer(Geometry const & /*geo*/, double /*delta*/);
