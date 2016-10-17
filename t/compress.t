use v5.20.0;
use warnings;

use Digest::SHA1;
use Test::More;

plan skip_all => "no executable detab in cwd" unless -x 'detab';

subtest "doesn't compress" => sub {
  my $result = `./compress corpus/uncompressable.txt`;

  my $in_len = -s 'corpus/uncompressable.txt';

  my $markers = $in_len / 255;
  $markers = int $markers + 1 if $markers > int $markers;

  is(
    length $result,
    $in_len + $markers,
    "our uncompressable file got $markers larger",
  );

  for (0, 1, 2) {
    is(
      ord substr($result, $_ + $_*255, 1),
      255,
      "${_}th marker is 255",
    );
  }

  is(
    ord substr($result, 3 + 3*255, 1),
    2,
    "3th marker is 2",
  );
};

subtest "compresses" => sub {
  my $result = `./compress corpus/compressable.txt`;

  my $in_len = -s 'corpus/compressable.txt';

  cmp_ok(
    length $result,
    '>',
    length $in_len,
    "we compressed and it got smaller",
  );

  isnt(
    index($result, "\x{00}5\x{05}"),
    -1,
    "we have a run of five 5s",
  );

  isnt(
    index($result, "\x{00}9\x{09}"),
    -1,
    "we have a run of nine 9s",
  );

  isnt(
    index($result, "\x{00}u\x{08}"),
    -1,
    "we have a run of eight u's",
  );
};

done_testing;
