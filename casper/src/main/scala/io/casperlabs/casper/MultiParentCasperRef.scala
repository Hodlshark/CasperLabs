package io.casperlabs.casper

import cats.Monad
import cats.effect.Sync
import cats.effect.concurrent.Ref
import cats.implicits._
import io.casperlabs.shared.{Log, LogSource, MaybeCell}

object MultiParentCasperRef {

  type MultiParentCasperRef[F[_]] = MaybeCell[F, MultiParentCasper[F]]

  def apply[F[_]](implicit ev: MultiParentCasperRef[F]): MultiParentCasperRef[F] = ev

  def of[F[_]: Sync]: F[MultiParentCasperRef[F]] = MaybeCell.of[F, MultiParentCasper[F]]

  // For usage in tests only
  def unsafe[F[_]: Sync](casperRef: Option[MultiParentCasper[F]] = None): MultiParentCasperRef[F] =
    MaybeCell.unsafe[F, MultiParentCasper[F]](casperRef)

  def withCasper[F[_]: Monad: Log: MultiParentCasperRef, A](
      f: MultiParentCasper[F] => F[A],
      msg: String,
      default: F[A]
  ): F[A] =
    MultiParentCasperRef[F].get flatMap {
      case Some(casper) => f(casper)
      case None =>
        Log[F]
          .warn(s"$msg. Casper instance was not available yet.")
          .flatMap(_ => default)
    }
}
