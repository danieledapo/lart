#include "lart/src/geo/types.rs.h"

#include "clipper2/clipper.h"

static Path to_path(Clipper2Lib::Path64 const &path64, double precision, bool close)
{
    Path pp;
    pp.points.reserve(path64.size());
    for (auto p : path64)
        pp.points.push_back(V{p.x / precision, p.y / precision});

    if (close && !pp.points.empty() && pp.points[0] != pp.points.back())
    {
        V p = pp.points[0];
        pp.points.push_back(p);
    }

    return pp;
}

static Clipper2Lib::Path64 to_path64(Path const &path, double precision)
{
    Clipper2Lib::Path64 path64(path.points.size());

    for (size_t i = 0; i < path64.size(); ++i)
    {
        V p = path.points[i];
        path64[i] = Clipper2Lib::Point64{p.x * precision, p.y * precision};
    }

    return path64;
}

struct Clipper::pimpl
{
    Clipper2Lib::Clipper64 clipper;
    Clipper2Lib::FillRule fill_rule = Clipper2Lib::FillRule::NonZero;
    double precision = 1000;

    Geometry execute(Clipper2Lib::ClipType ct)
    {
        Clipper2Lib::PolyTree64 polytree;
        Clipper2Lib::Paths64 paths;

        bool ok = clipper.Execute(ct, fill_rule, polytree, paths);
        if (!ok)
            return Geometry{};

        Geometry geo;
        geo.paths.reserve(polytree.Count() + paths.size());

        for (auto const &path64 : paths)
            geo.paths.push_back(to_path(path64, precision, false));

        for (auto const &poly : polytree)
        {
            std::vector<Clipper2Lib::PolyPath64 *> stack;
            stack.push_back(poly.get());

            while (!stack.empty())
            {
                auto *polypath = stack.back();
                stack.pop_back();

                geo.paths.push_back(to_path(polypath->Polygon(), precision, true));

                geo.paths.reserve(geo.paths.size() + polypath->Count());
                stack.reserve(stack.size() + polypath->Count());
                for (auto const &child : *polypath)
                    stack.push_back(child.get());
            }
        }

        return geo;
    }
};

Clipper::Clipper()
    : impl(std::make_shared<pimpl>())
{
}

void Clipper::add_subject(Path const &polygon)
{
    Clipper2Lib::Path64 path64 = to_path64(polygon, impl->precision);

    if (!path64.empty() && path64[0] == path64.back())
        impl->clipper.AddSubject({path64});
    else
        impl->clipper.AddOpenSubject({path64});
}

void Clipper::add_clip(Path const &polygon)
{
    impl->clipper.AddClip({to_path64(polygon, impl->precision)});
}

Geometry Clipper::union_()
{
    return impl->execute(Clipper2Lib::ClipType::Union);
}

Geometry Clipper::intersection()
{
    return impl->execute(Clipper2Lib::ClipType::Intersection);
}

Geometry Clipper::difference()
{
    return impl->execute(Clipper2Lib::ClipType::Difference);
}

Geometry Clipper::symmetric_difference()
{
    return impl->execute(Clipper2Lib::ClipType::Xor);
}

std::unique_ptr<Clipper> new_clipper()
{
    return std::make_unique<Clipper>();
}

Geometry buffer(Geometry const &geo, double delta)
{
    Clipper2Lib::ClipperOffset off;
    double precision = 1000.0;

    for (auto const &path : geo.paths)
    {
        if (path.points.empty())
            continue;

        Clipper2Lib::EndType end_type = Clipper2Lib::EndType::Round;
        if (path.points[0] == path.points.back())
            end_type = Clipper2Lib::EndType::Polygon;
        off.AddPath(to_path64(path, precision), Clipper2Lib::JoinType::Round, end_type);
    }

    Clipper2Lib::Paths64 paths;
    off.Execute(delta * precision, paths);

    Geometry out;
    out.paths.reserve(paths.size());
    for (auto const &p : paths)
        out.paths.push_back(to_path(p, precision, true));

    return out;
}
