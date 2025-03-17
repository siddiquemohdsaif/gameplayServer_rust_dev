#include "TableGeometry.h"

const double TableGeometry::tableX = 0.3683; //21.5
const double TableGeometry::tableY = 0.3683; //10.5

const double TableGeometry::pocketDia = 0.0652;
const double TableGeometry::pocketOffset = -0.0033;
const double TableGeometry::PX = TableGeometry::tableX - 0.5 * pocketDia + pocketOffset;
const double TableGeometry::PY = TableGeometry::tableY - 0.5 * pocketDia + pocketOffset;
const double TableGeometry::cornerRadius = 0.5 * pocketDia;

const Pocket TableGeometry::pocketNW = Pocket(Vector3(-TableGeometry::PX, TableGeometry::PY, 0), TableGeometry::cornerRadius);
const Pocket TableGeometry::pocketSW = Pocket(Vector3(-TableGeometry::PX, -TableGeometry::PY, 0), TableGeometry::cornerRadius);
const Pocket TableGeometry::pocketNE = Pocket(Vector3(TableGeometry::PX, TableGeometry::PY, 0), TableGeometry::cornerRadius);
const Pocket TableGeometry::pocketSE = Pocket(Vector3(TableGeometry::PX, -TableGeometry::PY, 0), TableGeometry::cornerRadius);

const std::vector<Pocket> TableGeometry::pockets = {TableGeometry::pocketNW, TableGeometry::pocketSW, TableGeometry::pocketNE, TableGeometry::pocketSE};
