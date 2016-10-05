use v5.20.0;
use warnings;
use Test::More;

plan skip_all => "no executable wc in cwd" unless -x 'wc';

my @tests = (                  # lines, words, bytes
  [ 'empty-file.txt'        => [    0,    0,    0 ] ],
  [ 'excessive-blanks.txt'  => [   12,   37,  352 ] ],
  [ 'lorem-ipsum.txt'       => [    6,   69,  446 ] ],
  [ 'no-newline.txt'        => [    0,    5,   24 ] ],
  [ 'one-empty-line.txt'    => [    1,    0,    1 ] ],
);

my $num = qr/[0-9]+/;

for my $test (@tests) {
  my ($fn, $expect) = @$test;
  my $line = `./wc ./corpus/$fn`;
  chomp $line;
  my ($l, $w, $b, $rfn) = $line =~ /\A\s*($num)\s+($num)\s+($num)\s+(.+)\z/;

  subtest "wc $fn" => sub {
    is($l, $expect->[0], "line count");
    is($w, $expect->[1], "word count");
    is($b, $expect->[2], "byte count");
    is($rfn, "./corpus/$fn", "reported filename");
  };
}

done_testing;
